// Perspective/View matrix code from:
// https://glium.github.io/glium/book/tuto-10-perspective.html
// https://glium.github.io/glium/book/tuto-12-camera.html
// I don't understand anything but it works

use std::f32::consts::PI;

pub fn calculate_forward_direction(yaw: f32, pitch: f32) -> [f32; 3] {
  [
    yaw.cos() * pitch.cos(),
    pitch.sin(),
    yaw.sin() * pitch.cos(),
  ]
}

fn normalize_plane(mut plane: [f32; 4]) -> [f32; 4] {
  let mag = (plane[0] * plane[0] + plane[1] * plane[1] + plane[2] * plane[2]).sqrt();
  plane[0] = plane[0] / mag;
  plane[1] = plane[1] / mag;
  plane[2] = plane[2] / mag;
  plane[3] = plane[3] / mag;
  plane
}

pub struct Camera {
  pub yaw: f32,
  pub pitch: f32,
  pub position: [f32; 3],
  pub direction: [f32; 3],
  pub up: [f32; 3],
  pub fov: f32,
  pub znear: f32,
  pub zfar: f32,
  pub perspective_matrix: [[f32; 4]; 4],
}
impl Camera {
  /// Update camera direction based on yaw/pitch
  pub fn update_direction(&mut self) {
    self.direction = calculate_forward_direction(self.yaw, self.pitch);
  }
  pub fn forward(&mut self, amount: f32) {
    self.position[0] += self.direction[0] * amount;
    self.position[1] += self.direction[1] * amount;
    self.position[2] += self.direction[2] * amount;
  }

  pub fn view_matrix(&self) -> [[f32; 4]; 4] {
    let position = self.position;
    let direction = self.direction;
    let up = self.up;

    let f = {
      let f = direction;
      let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
      let len = len.sqrt();
      [f[0] / len, f[1] / len, f[2] / len]
    };
    let s = [up[1] * f[2] - up[2] * f[1],
         up[2] * f[0] - up[0] * f[2],
         up[0] * f[1] - up[1] * f[0]];
    let s_norm = {
      let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
      let len = len.sqrt();
      [s[0] / len, s[1] / len, s[2] / len]
    };
    let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
         f[2] * s_norm[0] - f[0] * s_norm[2],
         f[0] * s_norm[1] - f[1] * s_norm[0]];
    let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
         -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
         -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];
    [
      [s_norm[0], u[0], f[0], 0.0],
      [s_norm[1], u[1], f[1], 0.0],
      [s_norm[2], u[2], f[2], 0.0],
      [p[0], p[1], p[2], 1.0],
    ]
  }
  
  pub fn update_perspective_matrix(&mut self, target_dimensions: (u32, u32)) {
    let znear = self.znear;
    let zfar = self.zfar;
    let fov = self.fov;
    let (width, height) = target_dimensions;
    let aspect_ratio = height as f32 / width as f32;
    let f = 1.0 / (fov / 2.0).tan();
    self.perspective_matrix = [
      [f*aspect_ratio, 0.0, 0.0,                            0.0],
      [0.0,            f,   0.0,                            0.0],
      [0.0,            0.0, (zfar+znear)/(zfar-znear),      1.0],
      [0.0,            0.0, -(2.0*zfar*znear)/(zfar-znear), 0.0],
    ];
  }

  // https://www.flipcode.com/archives/Frustum_Culling.shtml
  // https://web.archive.org/web/20070226173353/https://www2.ravensoft.com/users/ggribb/plane%20extraction.pdf
  pub fn frustum_planes(&self, normalized: bool) -> [[f32; 4]; 6] {
    let mut p_planes = [[0.0_f32; 4]; 6];
    let matrix = self.perspective_matrix;
    
    // Left clipping plane
    p_planes[0][0] = matrix[3][0] + matrix[0][0];
    p_planes[0][1] = matrix[3][1] + matrix[0][1];
    p_planes[0][2] = matrix[3][2] + matrix[0][2];
    p_planes[0][3] = matrix[3][3] + matrix[0][3];
    // Right clipping plane
    p_planes[1][0] = matrix[3][0] - matrix[0][0];
    p_planes[1][1] = matrix[3][1] - matrix[0][1];
    p_planes[1][2] = matrix[3][2] - matrix[0][2];
    p_planes[1][3] = matrix[3][3] - matrix[0][3];
    // Top clipping plane
    p_planes[2][0] = matrix[3][0] - matrix[1][0];
    p_planes[2][1] = matrix[3][1] - matrix[1][1];
    p_planes[2][2] = matrix[3][2] - matrix[1][2];
    p_planes[2][3] = matrix[3][3] - matrix[1][3];
    // Bottom clipping plane
    p_planes[3][0] = matrix[3][0] + matrix[1][0];
    p_planes[3][1] = matrix[3][1] + matrix[1][1];
    p_planes[3][2] = matrix[3][2] + matrix[1][2];
    p_planes[3][3] = matrix[3][3] + matrix[1][3];
    // Near clipping plane
    p_planes[4][0] = matrix[3][0] + matrix[3][0];
    p_planes[4][1] = matrix[3][1] + matrix[3][1];
    p_planes[4][2] = matrix[3][2] + matrix[3][2];
    p_planes[4][3] = matrix[3][3] + matrix[3][3];
    // Far clipping plane
    p_planes[5][0] = matrix[3][0] - matrix[3][0];
    p_planes[5][1] = matrix[3][1] - matrix[3][1];
    p_planes[5][2] = matrix[3][2] - matrix[3][2];
    p_planes[5][3] = matrix[3][3] - matrix[3][3];

    //Normalize planes
    if normalized {
      for plane in &mut p_planes {
        *plane = normalize_plane(*plane);
      }
    }

    p_planes
  }

}
impl Default for Camera {
  fn default() -> Self {
    Self {
      position: [0., 0., 0.],
      direction: [0., 0., 0.],
      up: [0., 1., 0.],
      fov: PI / 3.,
      zfar: 1024.,
      znear: 0.1,
      yaw: 0.,
      pitch: 0.,
      perspective_matrix: [[0.; 4]; 4]
    }
  }
}
