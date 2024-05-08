use shipyard::{AllStoragesView, IntoIter, IntoWithId, Unique, UniqueView, View};
use kubi_shared::{entity::Entity, transform::Transform};
use crate::{
  camera::Camera, prefabs::GpuPrefabs, settings::GameSettings
};

use super::{camera_uniform::CameraUniformBuffer, depth::DepthTexture, RenderCtx};

mod pipeline;

#[derive(Unique)]
pub struct EntitiesRenderState {
  pub pipeline: wgpu::RenderPipeline,
}

pub fn init_entities_render_state(storages: AllStoragesView) {
  storages.add_unique(EntitiesRenderState {
    pipeline: storages.run(pipeline::init_entities_pipeline),
  });
}

// TODO: entity models
pub fn render_entities(
  ctx: &mut RenderCtx,
  state: UniqueView<EntitiesRenderState>,
  depth: UniqueView<DepthTexture>,
  prefabs: UniqueView<GpuPrefabs>,
  camera_ubo: UniqueView<CameraUniformBuffer>,
  camera: View<Camera>,
  settings: UniqueView<GameSettings>,
  entities: View<Entity>,
  transform: View<Transform>,
) {
  let mut rpass = ctx.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
    label: Some("rpass_draw_entities"),
    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
      view: ctx.surface_view,
      resolve_target: None,
      ops: wgpu::Operations {
        load: wgpu::LoadOp::Load,
        store: wgpu::StoreOp::Store,
      },
    })],
    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
      view: &depth.depth_view,
      depth_ops: Some(wgpu::Operations {
        load: wgpu::LoadOp::Load,
        store: wgpu::StoreOp::Store,
      }),
      stencil_ops: None,
    }),
    ..Default::default()
  });

  rpass.set_pipeline(&state.pipeline);
  rpass.set_bind_group(0, &prefabs.player_model_diffuse_bind_group, &[]);
  rpass.set_bind_group(1, &camera_ubo.camera_bind_group, &[]);
  rpass.set_vertex_buffer(0, prefabs.player_model.vertex.slice(..));
  rpass.set_index_buffer(prefabs.player_model.index.slice(..), wgpu::IndexFormat::Uint32);
  rpass.draw_indexed(0..prefabs.player_model.index_len, 0, 0..1);

  // let (camera_id, _camera) = camera.iter().with_id().next().expect("No cameras in the scene");

  // for (entity_id, (_, trans)) in (&entities, &transform).iter().with_id() {
  //   //skip rendering camera holder (as the entity would block the view)
  //   if entity_id == camera_id { continue }

  // }
}
