use glium::Display;
use glutin::event_loop::EventLoop;

pub fn init_display(event_loop: &EventLoop<()>) -> Display {
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    Display::new(wb, cb, &event_loop).unwrap()
}
