use shipyard::{Unique, NonSendSync, UniqueView, UniqueViewMut, View, IntoIter};
use glium::{
  Display, Surface, uniform,
  DrawParameters, 
  uniforms::{
    Sampler, 
    SamplerBehavior, 
    MinifySamplerFilter, 
    MagnifySamplerFilter
  },
  draw_parameters::{
    Depth,
    DepthTest,
    PolygonMode,
    BackfaceCullingMode,
  },
  glutin::{
    event_loop::EventLoop, 
    window::WindowBuilder, 
    ContextBuilder, GlProfile
  }, 
};
use glam::Vec3;
use crate::{
  camera::Camera, 
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

#[derive(Unique)]
pub struct RenderTarget(pub glium::Frame);

#[derive(Unique)]
pub struct BackgroundColor(pub Vec3);

#[derive(Unique)]
pub struct Renderer {
  pub display: Display
}
impl Renderer {
  pub fn init(event_loop: &EventLoop<()>) -> Self {
    log::info!("initializing display");
    let wb = WindowBuilder::new()
      .with_title("uwu")
      .with_maximized(true);
    let cb = ContextBuilder::new()
      .with_depth_buffer(24)
      .with_gl_profile(GlProfile::Core);
    let display = Display::new(wb, cb, event_loop)
      .expect("Failed to create a glium Display");
    Self { display }
  }
}

pub fn clear_background(
  mut target: NonSendSync<UniqueViewMut<RenderTarget>>, 
  color: UniqueView<BackgroundColor>,
) {
  target.0.clear_color_srgb_and_depth((color.0.x, color.0.y, color.0.z, 1.), 1.);
}

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
    backface_culling: BackfaceCullingMode::CullCounterClockwise,
    ..Default::default()
  };
  let texture_sampler = Sampler(&texture.0, SamplerBehavior {
    minify_filter: MinifySamplerFilter::Linear,
    magnify_filter: MagnifySamplerFilter::Nearest,
    max_anisotropy: 8,
    ..Default::default()
  });
  let view = camera.view_matrix.to_cols_array_2d();
  let perspective = camera.perspective_matrix.to_cols_array_2d();

  for (&position, chunk) in &chunks.chunks {
    if let Some(key) = chunk.mesh_index {
      let mesh = meshes.get(key).expect("Mesh index pointing to nothing");
      let world_position = (position.as_vec3() * CHUNK_SIZE as f32).to_array();
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
