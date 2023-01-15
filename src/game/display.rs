use glium::Display;
use glium::glutin::{
  ContextBuilder, 
  window::WindowBuilder,
  event_loop::EventLoop
};

pub fn init_display(event_loop: &EventLoop<()>) -> Display {
    let wb = WindowBuilder::new();
    let cb = ContextBuilder::new().with_depth_buffer(24);
    Display::new(wb, cb, &event_loop).unwrap()
}
