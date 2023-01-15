// Perspective/View matrix code from:
// https://glium.github.io/glium/book/tuto-10-perspective.html
// https://glium.github.io/glium/book/tuto-12-camera.html
// I don't understand anything but it works

use std::f32::consts::PI;

pub struct Camera {
    pub position: [f32; 3],
    pub direction: [f32; 3],
    pub up: [f32; 3],
    pub fov: f32,
    pub znear: f32,
    pub zfar: f32,
}
impl Camera {
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
    
    pub fn perspective_matrix(&self, target_dimensions: (u32, u32)) -> [[f32; 4]; 4] {
        let znear = self.znear;
        let zfar = self.zfar;
        let fov = self.fov;
        let (width, height) = target_dimensions;
        let aspect_ratio = height as f32 / width as f32;
        let f = 1.0 / (fov / 2.0).tan();
        [
            [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
            [         0.0         ,     f ,              0.0              ,   0.0],
            [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
            [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
        ]
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
        }
    }
}
