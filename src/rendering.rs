use shipyard::{Unique, NonSendSync, UniqueView, UniqueViewMut};
use glium::{
  glutin::{
    event_loop::EventLoop, 
    window::WindowBuilder, 
    ContextBuilder, GlProfile
  },
  Display, Surface,
};
use glam::Vec3;

#[derive(Unique)]
pub struct RenderTarget(pub glium::Frame);

#[derive(Unique)]
pub struct BackgroundColor(pub Vec3);

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

pub fn clear_background(mut target: NonSendSync<UniqueViewMut<RenderTarget>>, color: UniqueView<BackgroundColor>) {
  target.0.clear_color_srgb_and_depth((color.0.x, color.0.y, color.0.z, 1.), 1.);
}