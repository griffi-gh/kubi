use shipyard::{View, IntoIter, NonSendSync, UniqueViewMut};
use crate::{
  player::MainPlayer, 
  world::raycast::LookingAtBlock, 
  camera::Camera
};
use super::RenderTarget;

//wip
pub fn render_selection_box(
  lookat: View<LookingAtBlock>,
  camera: View<Camera>,
  mut target: NonSendSync<UniqueViewMut<RenderTarget>>, 
) {
  for lookat in lookat.iter() {
    
  }
}
