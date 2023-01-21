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
pub(crate) mod transform;
pub(crate) mod settings;

use rendering::{Rederer, RenderTarget, BackgroundColor, clear_background};
use player::spawn_player;
use world::{GameWorld, loading::load_world_around_player};
use prefabs::load_prefabs;
use settings::GameSettings;

#[derive(Unique)]
pub(crate) struct DeltaTime(Duration);

fn startup() -> Workload {
  (
    spawn_player,
  ).into_workload()
}
fn update() -> Workload {
  (
    load_world_around_player
  ).into_workload()
}
fn render() -> Workload {
  (
    clear_background,
  ).into_workload()
}

fn main() {
  logging::init();

  //Create event loop
  let event_loop = EventLoop::new();

  //Create a shipyard world
  let world = World::new();

  //Add systems and uniques, Init and load things
  world.add_unique_non_send_sync(
    Rederer::init(&event_loop)
  );
  load_prefabs(&world);
  world.add_unique_non_send_sync(
    GameWorld::new()
  );
  world.add_unique(BackgroundColor(vec3(0.5, 0.5, 1.)));
  world.add_unique(DeltaTime(Duration::default()));
  world.add_unique(GameSettings::default());

  //Register workloads
  world.add_workload(startup);
  world.add_workload(update);
  world.add_workload(render);

  //Run startup systems
  world.run_workload(startup).unwrap();

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
