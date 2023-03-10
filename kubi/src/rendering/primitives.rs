use shipyard::{Workload, IntoWorkload};
use glium::implement_vertex;

pub mod cube;
pub mod rect;

use cube::init_cube_primitive;
use rect::init_rect_primitive;

#[derive(Clone, Copy, Default)]
pub struct PositionOnlyVertex {
  pub position: [f32; 3],
}
implement_vertex!(PositionOnlyVertex, position);

#[derive(Clone, Copy, Default)]
pub struct PositionOnlyVertex2d {
  pub position: [f32; 2],
}
implement_vertex!(PositionOnlyVertex2d, position);

pub fn init_primitives() -> Workload {
  (
    init_cube_primitive,
    init_rect_primitive,
  ).into_sequential_workload()
}
