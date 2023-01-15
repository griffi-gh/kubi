use glium::Display;
use glium::glutin::{
  ContextBuilder,
  GlProfile,
  window::WindowBuilder,
  event_loop::EventLoop
};

pub fn init_display(event_loop: &EventLoop<()>) -> Display {
  let wb = WindowBuilder::new()
    .with_maximized(true);
  let cb = ContextBuilder::new()
    .with_depth_buffer(24)
    .with_gl_profile(GlProfile::Core);
  Display::new(wb, cb, event_loop).unwrap()
}
