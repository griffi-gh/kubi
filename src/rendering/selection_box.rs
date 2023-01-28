use shipyard::{View, IntoIter, NonSendSync, UniqueViewMut};
use glium::{Surface, implement_vertex};
use crate::{
  world::raycast::LookingAtBlock, 
  camera::Camera
};
use super::RenderTarget;

#[derive(Clone, Copy)]
pub struct SelBoxVertex {
  pub position: [f32; 3],
}
implement_vertex!(SelBoxVertex, position);

//wip
pub fn render_selection_box(
  lookat: View<LookingAtBlock>,
  camera: View<Camera>,
  mut target: NonSendSync<UniqueViewMut<RenderTarget>>, 
) {
  for lookat in lookat.iter() {
    // target.0.draw(
    //   &mesh.vertex_buffer,
    //   &mesh.index_buffer,
    //   &program.0,
    //   &uniform! {
    //     position_offset: world_position.to_array(),
    //     view: view,
    //     perspective: perspective,
    //     tex: texture_sampler,
    //   },
    //   &draw_parameters
    // ).unwrap();
  }
}
