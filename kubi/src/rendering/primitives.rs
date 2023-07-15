use shipyard::{Workload, IntoWorkload};

pub mod cube;
pub mod rect;

use cube::init_cube_primitive;
use rect::init_rect_primitive;

#[repr(C, packed)]
#[derive(Clone, Copy, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PositionVertex {
  pub position: [f32; 3],
}

#[repr(C, packed)]
#[derive(Clone, Copy, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PositionVertex2d {
  pub position: [f32; 2],
}

pub fn init_primitives() -> Workload {
  (
    init_cube_primitive,
    init_rect_primitive,
  ).into_sequential_workload()
}
