use glam::Vec3;
use shipyard::{AllStoragesView, IntoIter, NonSendSync, Unique, UniqueView, View};
use kubi_shared::chunk::CHUNK_SIZE;
use crate::{
  camera::Camera,
  prefabs::TexturePrefabs,
  world::{ChunkMeshStorage, ChunkStorage},
};
use super::{camera::CameraUniformBuffer, depth::DepthTexture, RenderCtx};

mod pipeline;
mod vertex;
pub use vertex::ChunkVertex;

#[derive(Unique)]
pub struct WorldRenderState {
  pub pipeline: wgpu::RenderPipeline,
  //pub trans_chunk_queue: Vec<IVec3>,
}

pub fn init_world_render_state(storages: AllStoragesView) {
  storages.add_unique(WorldRenderState {
    pipeline: storages.run(pipeline::init_world_pipeline),
    //trans_chunk_queue: Vec::with_capacity(512),
  })
}

pub fn draw_world(
  ctx: &mut RenderCtx,
  state: UniqueView<WorldRenderState>,
  camera_ubo: UniqueView<CameraUniformBuffer>,
  depth: UniqueView<DepthTexture>,
  textures: UniqueView<TexturePrefabs>,
  camera: View<Camera>,
  chunks: UniqueView<ChunkStorage>,
  meshes: NonSendSync<UniqueView<ChunkMeshStorage>>,
  //settings: UniqueView<GameSettings>,
) {
  let camera = camera.iter().next().expect("No cameras in the scene");

  let mut render_pass = ctx.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
    label: Some("draw_world"),
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
        load: wgpu::LoadOp::Clear(1.0),
        store: wgpu::StoreOp::Store,
      }),
      stencil_ops: None,
    }),
    ..Default::default()
  });

  render_pass.set_pipeline(&state.pipeline);
  render_pass.set_bind_group(0, &textures.block_diffuse_bind_group, &[]);
  render_pass.set_bind_group(1, &camera_ubo.camera_bind_group, &[]);

  for (&position, chunk) in &chunks.chunks {
    if let Some(key) = chunk.mesh_index {
      let mesh = meshes.get(key).expect("Mesh index pointing to nothing");
      let world_position = position.as_vec3() * CHUNK_SIZE as f32;

      //Skip if mesh is empty
      if mesh.main.index.size() == 0 && mesh.trans.index.size() == 0 {
        continue
      }

      //Frustum culling
      let minp = world_position;
      let maxp = world_position + Vec3::splat(CHUNK_SIZE as f32);
      if !camera.frustum.is_box_visible(minp, maxp) {
        continue
      }

      //Draw chunk mesh
      if mesh.main.index.size() > 0 {
        render_pass.set_index_buffer(mesh.main.index.slice(..), wgpu::IndexFormat::Uint32);
        render_pass.set_vertex_buffer(0, mesh.main.vertex.slice(..));
        render_pass.draw_indexed(0..mesh.main.index_len, 0, 0..1);
      }

      //TODO trans chunks
      // if mesh.trans_index_buffer.len() > 0 {
      //   trans_queue.0.push(position);
      // }
    }
  }
}
