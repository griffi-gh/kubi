use shipyard::{Unique, NonSendSync, UniqueView, UniqueViewMut, AllStoragesView};
use glium::{
  Display, Surface, 
  Version, Api,
  glutin::{
    event_loop::EventLoop, 
    window::WindowBuilder, 
    ContextBuilder, GlProfile
  }, 
};
use glam::{Vec3, UVec2};

pub mod primitives;
pub mod world;
pub mod selection_box;

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
    log::info!("Renderer: {}", display.get_opengl_renderer_string());
    log::info!("OpenGL {}", display.get_opengl_version_string());
    log::info!("Supports GLES {:?}", display.get_supported_glsl_version());
    assert!(display.is_glsl_version_supported(&Version(Api::GlEs, 3, 0)), "GLES 3.0 is not supported");
    Self { display }
  }
}

pub fn clear_background(
  mut target: NonSendSync<UniqueViewMut<RenderTarget>>, 
  color: UniqueView<BackgroundColor>,
) {
  target.0.clear_color_srgb_and_depth((color.0.x, color.0.y, color.0.z, 1.), 1.);
}
