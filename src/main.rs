use shipyard::{
  World, Workload, IntoWorkload, 
  UniqueView, UniqueViewMut, 
  NonSendSync, Unique
};
use glium::{
  glutin::{
    event_loop::{EventLoop, ControlFlow},
    event::{Event, WindowEvent}
  }
};
use glam::vec3;
use std::time::{Instant, Duration};

mod logging;

pub(crate) mod rendering;
pub(crate) mod world;
pub(crate) mod player;
pub(crate) mod prefabs;
pub(crate) mod transform;
pub(crate) mod settings;
pub(crate) mod camera;
pub(crate) mod events;
pub(crate) mod input;
pub(crate) mod fly_controller;

use rendering::{
  Renderer, 
  RenderTarget, 
  BackgroundColor, 
  clear_background
};
use world::{
  init_game_world,
  loading::update_loaded_world_around_player, 
  raycast::update_player_raycast
};
use player::spawn_player;
use prefabs::load_prefabs;
use settings::GameSettings;
use camera::compute_cameras;
use events::{clear_events, process_glutin_events};
use input::{init_input, process_inputs};
use fly_controller::update_controllers;
use rendering::{
  selection_box::render_selection_box,
  world::draw_world,
};

#[derive(Unique)]
pub(crate) struct DeltaTime(Duration);

fn startup() -> Workload {
  (
    load_prefabs,
    init_input,
    init_game_world,
    spawn_player,
  ).into_workload()
}
fn update() -> Workload {
  (
    process_inputs,
    update_controllers,
    update_loaded_world_around_player,
    update_player_raycast,
    compute_cameras
  ).into_workload()
}
fn render() -> Workload {
  (
    clear_background,
    draw_world,
  ).into_sequential_workload()
}
fn after_frame_end() -> Workload {
  (
    clear_events,
  ).into_workload()
}

fn main() {
  logging::init();

  //Create event loop
  let event_loop = EventLoop::new();

  //Create a shipyard world
  let mut world = World::new();

  //Add systems and uniques, Init and load things
  world.add_unique_non_send_sync(Renderer::init(&event_loop));
  world.add_unique(BackgroundColor(vec3(0.5, 0.5, 1.)));
  world.add_unique(DeltaTime(Duration::default()));
  world.add_unique(GameSettings::default());

  //Register workloads
  world.add_workload(startup);
  world.add_workload(update);
  world.add_workload(render);
  world.add_workload(after_frame_end);

  //Run startup systems
  world.run_workload(startup).unwrap();

  //Run the event loop
  let mut last_update = Instant::now();
  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Poll;
    process_glutin_events(&mut world, &event);
    match event {
      Event::WindowEvent { event, .. } => match event {
        WindowEvent::Resized(_size) => {
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
        let target = {
          let renderer = world.borrow::<NonSendSync<UniqueView<Renderer>>>().unwrap();
          renderer.display.draw()
        };
        world.add_unique_non_send_sync(RenderTarget(target));

        //Run render workflow
        world.run_workload(render).unwrap();

        //Finish rendering
        let target = world.remove_unique::<RenderTarget>().unwrap(); 
        target.0.finish().unwrap();

        //FrameEnd
        world.run_workload(after_frame_end).unwrap();
      },
      _ => (),
    };
  });
}
