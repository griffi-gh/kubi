use glium::glutin::event_loop::EventLoop;
use shipyard::World;

mod logging;

fn main() {
  logging::init();
  
  let world = World::new();
  world.add_unique(component)

  let event_loop = EventLoop::new();
  event_loop.run(move |event, _, control_flow| {
    
  });
}
