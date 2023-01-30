use glam::Mat4;
use shipyard::{View, IntoIter, NonSendSync, UniqueViewMut, UniqueView};
use glium::{
  Surface, 
  DrawParameters, 
  BackfaceCullingMode, 
  Blend, Depth, DepthTest,
  uniform, 
};
use crate::{
  world::raycast::LookingAtBlock, 
  camera::Camera, prefabs::SelBoxShaderPrefab
};
use super::{
  RenderTarget, 
  primitives::SimpleBoxBuffers,
};

pub fn render_selection_box(
  lookat: View<LookingAtBlock>,
  camera: View<Camera>,
  mut target: NonSendSync<UniqueViewMut<RenderTarget>>, 
  program: NonSendSync<UniqueView<SelBoxShaderPrefab>>,
  buffers: NonSendSync<UniqueView<SimpleBoxBuffers>>,
) {
  let camera = camera.iter().next().unwrap();
  let Some(lookat) = lookat.iter().next() else { return };
  let Some(lookat) = lookat.0 else { return };

  //Darken block
  target.0.draw(
    &buffers.0,
    &buffers.1,
    &program.0,
    &uniform! {
      u_color: [0., 0., 0., 0.5_f32],
      u_position: lookat.block_position.to_array(),
      perspective: camera.perspective_matrix.to_cols_array_2d(),
      view: camera.view_matrix.to_cols_array_2d(),
    },
    &DrawParameters {
      backface_culling: BackfaceCullingMode::CullClockwise,
      blend: Blend::alpha_blending(),
      depth: Depth {
        test: DepthTest::IfLessOrEqual, //this may be unreliable!
        ..Default::default()
      },
      ..Default::default()
    }
  ).unwrap();
}
