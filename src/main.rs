use shipyard::{
  World, Workload, IntoWorkload, 
  UniqueView, UniqueViewMut, 
  NonSendSync, Unique
};
use glium::{
  Surface,
  glutin::{
    event_loop::{EventLoop, ControlFlow},
    event::{Event, WindowEvent}
  }
};
use glam::vec3;
use std::time::{Instant, Duration};

mod logging;
pub(crate) mod rendering;
pub(crate) mod player;
pub(crate) mod world;
pub(crate) mod prefabs;

use rendering::{Rederer, RenderTarget, BackgroundColor, clear_background};
use prefabs::load_prefabs;

#[derive(Unique)]
pub(crate) struct DeltaTime(Duration);

fn render() -> Workload {
  (
    clear_background,

  ).into_workload()
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

  //Init and load things
  world.add_unique_non_send_sync(
    Rederer::init(&event_loop)
  );
  load_prefabs(&world);

  //Add systems and uniques
  world.add_unique(BackgroundColor(vec3(0.5, 0.5, 1.)));
  world.add_unique(DeltaTime(Duration::default()));
  world.add_workload(update);
  world.add_workload(render);

  //Run the event loop
  let mut last_update = Instant::now();
  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Poll;
    match event {
      Event::WindowEvent { event, .. } => match event {
        WindowEvent::Resized(size) => {
          // todo ...
        }
        WindowEvent::CloseRequested => {
          log::info!("exit requested");
          *control_flow = ControlFlow::Exit;
        },
        _ => (),
      },
      Event::MainEventsCleared => {
        //Update delta time (maybe move this into a system?)
        {
          let mut dt_view = world.borrow::<UniqueViewMut<DeltaTime>>().unwrap();
          let now = Instant::now();
          dt_view.0 = now - last_update;
          last_update = now;
        }
        
        //Run update workflow
        world.run_workload(update).unwrap();

        //Start rendering (maybe use custom views for this?)
        let mut target = {
          let renderer = world.borrow::<NonSendSync<UniqueView<Rederer>>>().unwrap();
          renderer.display.draw()
        };
        target.clear_color_and_depth((0., 0., 0., 1.), 1.);
        world.add_unique_non_send_sync(RenderTarget(target));

        //Run render workflow
        world.run_workload(render).unwrap();

        //Finish rendering
        let target = world.remove_unique::<RenderTarget>().unwrap(); 
        target.0.finish().unwrap();
      },
      _ => (),
    };
  });
}
