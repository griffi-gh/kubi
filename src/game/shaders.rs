use glium::{Display, Program};

pub mod chunk;
pub mod colored2d;

pub struct Programs {
  pub colored_2d: Program,
  pub chunk: Program,
}
impl Programs {
  pub fn compile_all(display: &Display) -> Self {
    Self {
      colored_2d: Program::from_source(display, colored2d::VERTEX_SHADER, colored2d::FRAGMENT_SHADER, None).unwrap(),
      chunk: Program::from_source(display, chunk::VERTEX_SHADER, chunk::FRAGMENT_SHADER, None).unwrap(),
    }
  }
}
