use glium::{uniform, Blend, DrawParameters, Surface};
use kubi_shared::transform::Transform;
use shipyard::{IntoIter, NonSendSync, UniqueView, UniqueViewMut, View};
use crate::{
  player::MainPlayer,
  prefabs::Colored2ShaderPrefab,
  rendering::primitives::stri::STriPrimitive,
  world::ChunkStorage,
};
use super::RenderTarget;

pub fn render_submerged_view(
  mut target: NonSendSync<UniqueViewMut<RenderTarget>>,
  primitive: NonSendSync<UniqueView<STriPrimitive>>,
  program: NonSendSync<UniqueView<Colored2ShaderPrefab>>,
  plr: View<MainPlayer>,
  trans: View<Transform>,
  world: UniqueView<ChunkStorage>,
) {
  let (_, plr_trans) = (&plr, &trans).iter().next().expect("Main player MIA");
  let plr_pos = plr_trans.0.to_scale_rotation_translation().2;
  let block_at_pos = world.get_block(plr_pos.floor().as_ivec3());
  let Some(block_at_pos) = block_at_pos  else { return };
  let Some(color) = block_at_pos.descriptor().submerge else { return };

  let draw_parameters = DrawParameters {
    blend: Blend::alpha_blending(),
    ..Default::default()
  };
  target.0.draw(
    &primitive.0,
    &primitive.1,
    &program.0,
    &uniform! {
      color: color.to_array(),
    },
    &draw_parameters,
  ).unwrap();
}
