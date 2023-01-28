use shipyard::{View, IntoIter, NonSendSync, UniqueViewMut};
use glium::Surface;
use crate::{
  world::raycast::LookingAtBlock, 
  camera::Camera
};
use super::RenderTarget;

const CUBE_VERTICES: &[f32] = &[0.0];

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
