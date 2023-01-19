use shipyard::{World, Workload, IntoWorkload, View, UniqueView, NonSendSync};
use glium::{
  Surface,
  glutin::{
    event_loop::{EventLoop, ControlFlow},
    event::{Event, WindowEvent}
  }
};

mod logging;
mod rendering;

use rendering::{Rederer, RenderTarget};

fn render() -> Workload {
  (||()).into_workload()
}
fn update() -> Workload {
  (||()).into_workload()
}

fn main() {
  logging::init();

  //Create event loop
  let event_loop = EventLoop::new();

  //Create a shipyard world
  let world = World::new();

  //Add systems and uniques
  world.add_unique_non_send_sync(
    Rederer::init(&event_loop)
  );
  world.add_workload(update);
  world.add_workload(render);

  //Run the event loop
  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Poll;
    match event {
      Event::WindowEvent { event, .. } => match event {
        WindowEvent::CloseRequested => {
          *control_flow = ControlFlow::Exit;
        },
        _ => (),
      },
      Event::MainEventsCleared => {
        world.run_workload(update).unwrap();
        let mut target = {
          let renderer = world.borrow::<NonSendSync<UniqueView<Rederer>>>().unwrap();
          renderer.display.draw()
        };
        target.clear_color_and_depth((0., 0., 0., 1.), 1.);
        world.add_unique_non_send_sync(RenderTarget(target));
        world.run_workload(render).unwrap();
        let target = world.remove_unique::<RenderTarget>().unwrap(); 
        target.0.finish().unwrap();
      },
      _ => (),
    };
  });
}
