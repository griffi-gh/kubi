use shipyard::{NonSendSync, UniqueView, UniqueViewMut, View, IntoIter};
use glium::{
  implement_vertex, uniform,
  Surface, DrawParameters, 
  uniforms::{
    Sampler, 
    SamplerBehavior, 
    MinifySamplerFilter, 
    MagnifySamplerFilter, 
    SamplerWrapFunction
  },
  draw_parameters::{
    Depth,
    DepthTest,
    PolygonMode,
    BackfaceCullingMode,
  }
};
use crate::{
  camera::Camera, 
  rendering::RenderTarget,
  prefabs::{
    ChunkShaderPrefab,
    BlockTexturesPrefab,
  },
  world::{
    ChunkStorage, 
    ChunkMeshStorage, 
    chunk::CHUNK_SIZE,
  }, 
};

#[derive(Clone, Copy)]
pub struct ChunkVertex {
  pub position: [f32; 3],
  pub normal: [f32; 3],
  pub uv: [f32; 2],
  pub tex_index: u8,
}
implement_vertex!(ChunkVertex, position, normal, uv, tex_index);


pub fn draw_world(
  mut target: NonSendSync<UniqueViewMut<RenderTarget>>, 
  chunks: UniqueView<ChunkStorage>,
  meshes: NonSendSync<UniqueView<ChunkMeshStorage>>,
  program: NonSendSync<UniqueView<ChunkShaderPrefab>>,
  texture: NonSendSync<UniqueView<BlockTexturesPrefab>>,
  camera: View<Camera>,
) {
  let camera = camera.iter().next().expect("No cameras in the scene");
  let draw_parameters = DrawParameters {
    depth: Depth {
      test: DepthTest::IfLess,
      write: true,
      ..Default::default()
    },
    polygon_mode: PolygonMode::Fill, //Change to Line for wireframe
    backface_culling: BackfaceCullingMode::CullClockwise,
    ..Default::default()
  };
  let texture_sampler = Sampler(&texture.0, SamplerBehavior {
    minify_filter: MinifySamplerFilter::LinearMipmapLinear,
    magnify_filter: MagnifySamplerFilter::Nearest,
    max_anisotropy: 8,
    wrap_function: (SamplerWrapFunction::Clamp, SamplerWrapFunction::Clamp, SamplerWrapFunction::Clamp),
    ..Default::default()
  });
  let view = camera.view_matrix.to_cols_array_2d();
  let perspective = camera.perspective_matrix.to_cols_array_2d();

  for (&position, chunk) in &chunks.chunks {
    if let Some(key) = chunk.mesh_index {
      let mesh = meshes.get(key).expect("Mesh index pointing to nothing");
      let world_position = (position.as_vec3() * CHUNK_SIZE as f32).to_array();
      if mesh.index_buffer.len() > 0 { //maybe this is a bit hacky?
        target.0.draw(
          &mesh.vertex_buffer,
          &mesh.index_buffer,
          &program.0,
          &uniform! {
            position_offset: world_position,
            view: view,
            perspective: perspective,
            tex: texture_sampler
          },
          &draw_parameters
        ).unwrap();
      }
    }
  }
}
