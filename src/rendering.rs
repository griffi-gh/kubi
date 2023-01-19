use shipyard::Unique;
use glium::{
  glutin::{
    event_loop::EventLoop, 
    window::WindowBuilder, 
    ContextBuilder, GlProfile
  },
  Display,
};

#[derive(Unique)]
pub struct RenderTarget(pub glium::Frame);

#[derive(Unique)]
pub struct Rederer {
  pub display: Display
}
impl Rederer {
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
