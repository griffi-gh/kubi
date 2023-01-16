use glium::glutin::event::{VirtualKeyCode, ElementState};
use std::f32::consts::PI;
use crate::game::camera::Camera;

#[derive(Default, Clone, Copy)]
pub struct InputAmounts {
  move_x: (f32, f32),
  move_y: (f32, f32),
  move_z: (f32, f32),
  look_h: f32,
  look_v: f32,
}

pub struct Actions {
  pub movement: [f32; 3],
  pub rotation: [f32; 2],
}
impl Actions {
  pub fn apply_to_camera(&self, camera: &mut Camera) {
    //Apply rotation
    camera.yaw -= self.rotation[0];
    camera.pitch -= self.rotation[1];
    camera.pitch = camera.pitch.clamp(-PI/2. + f32::EPSILON, PI/2. - f32::EPSILON);
    camera.update_direction();
    //Apply movement
    let (yaw_sin, yaw_cos) = camera.yaw.sin_cos();
    //forward movement
    camera.position[0] += yaw_cos * self.movement[2];
    camera.position[2] += yaw_sin * self.movement[2];
    //sideways movement
    camera.position[0] -= -yaw_sin * self.movement[0];
    camera.position[2] -= yaw_cos * self.movement[0];
    //up/down movement
    camera.position[1] += self.movement[1];
  }
}

pub struct Controls {
  inputs: InputAmounts,
  pub speed: f32,
  pub sensitivity: f32,
}
impl Controls {
  //TODO locking controls
  pub fn lock(&mut self) {
    todo!()
  }
  pub fn unlock(&mut self) {
    todo!()
  }
  pub fn process_mouse_input(&mut self, dx: f64, dy: f64) {
    self.inputs.look_h += dx as f32;
    self.inputs.look_v += dy as f32;
  }
  pub fn process_keyboard_input(&mut self, key: VirtualKeyCode, state: ElementState) {
    let value = match state { 
      ElementState::Pressed => 1.,
      ElementState::Released => 0.,
    };
    match key {
      VirtualKeyCode::W | VirtualKeyCode::Up => {
        self.inputs.move_z.0 = value;
      }
      VirtualKeyCode::S | VirtualKeyCode::Down => {
        self.inputs.move_z.1 = -value;
      }
      VirtualKeyCode::A | VirtualKeyCode::Left => {
        self.inputs.move_x.0 = -value;
      }
      VirtualKeyCode::D | VirtualKeyCode::Right => {
        self.inputs.move_x.1 = value;
      }
      VirtualKeyCode::Space => {
        self.inputs.move_y.0 = value;
      }
      VirtualKeyCode::LShift => {
        self.inputs.move_y.1 = -value;
      }
      _ => ()
    }
  }
  pub fn calculate(&mut self, dt: f32) -> Actions {
    let movement = {
      let move_x = self.inputs.move_x.0 + self.inputs.move_x.1;
      let move_y = self.inputs.move_y.0 + self.inputs.move_y.1;
      let move_z = self.inputs.move_z.0 + self.inputs.move_z.1;
      let magnitude = (move_x.powi(2) + move_y.powi(2) + move_z.powi(2)).sqrt();
      if magnitude == 0. {
        [0., 0., 0.]
      } else {
        [
          dt * self.speed * (move_x / magnitude), 
          dt * self.speed * (move_y / magnitude), 
          dt * self.speed * (move_z / magnitude)
        ]
      }
    };
    let rotation = [
      dt * self.inputs.look_h * self.sensitivity,
      dt * self.inputs.look_v * self.sensitivity
    ];
    //Only mouse related actions need to be reset
    self.inputs.look_h = 0.;
    self.inputs.look_v = 0.;
    Actions { movement, rotation }
  }
}
impl Default for Controls {
  fn default() -> Self {
    Self {
      inputs: Default::default(),
      speed: 1.,
      sensitivity: 2.,
    }
  }
}
