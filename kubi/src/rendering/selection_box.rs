use glam::{Mat4, Vec3, Quat};
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
  camera::Camera, 
  prefabs::ColoredShaderPrefab
};
use super::{
  RenderTarget, 
  primitives::cube::CubePrimitive,
};

const SMOL: f32 = 0.0025;

pub fn render_selection_box(
  lookat: View<LookingAtBlock>,
  camera: View<Camera>,
  mut target: NonSendSync<UniqueViewMut<RenderTarget>>, 
  program: NonSendSync<UniqueView<ColoredShaderPrefab>>,
  buffers: NonSendSync<UniqueView<CubePrimitive>>,
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
      color: [0., 0., 0., 0.5_f32],
      model: Mat4::from_scale_rotation_translation(
        Vec3::splat(1. + SMOL * 2.),
        Quat::default(),
        lookat.block_position.as_vec3() - Vec3::splat(SMOL)
      ).to_cols_array_2d(),
      perspective: camera.perspective_matrix.to_cols_array_2d(),
      view: camera.view_matrix.to_cols_array_2d(),
    },
    &DrawParameters {
      backface_culling: BackfaceCullingMode::CullClockwise,
      blend: Blend::alpha_blending(),
      depth: Depth {
        //this may be unreliable... unless scale is applied! hacky...
        test: DepthTest::IfLessOrEqual, 
        ..Default::default()
      },
      ..Default::default()
    }
  ).unwrap();
}
