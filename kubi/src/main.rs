// allowed because systems often need a lot of arguments
#![allow(clippy::too_many_arguments)] 

use shipyard::{
  World, Workload, IntoWorkload, 
  UniqueView, UniqueViewMut, 
  NonSendSync
};
use glium::{
  glutin::{
    event_loop::{EventLoop, ControlFlow},
    event::{Event, WindowEvent}
  }
};
use glam::vec3;
use std::time::Instant;

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
pub(crate) mod block_placement;
pub(crate) mod delta_time;
pub(crate) mod cursor_lock;
pub(crate) mod control_flow;
pub(crate) mod state;
pub(crate) mod gui;

use world::{
  init_game_world,
  loading::update_loaded_world_around_player, 
  raycast::update_raycasts, queue::apply_queued_blocks
};
use player::spawn_player;
use prefabs::load_prefabs;
use settings::load_settings;
use camera::compute_cameras;
use events::{
  clear_events, process_glutin_events, 
  player_actions::generate_move_events
};
use input::{init_input, process_inputs};
use fly_controller::update_controllers;
use rendering::{
  Renderer, 
  RenderTarget, 
  BackgroundColor, 
  clear_background,
  primitives::init_simple_box_buffers,
  selection_box::render_selection_box,
  world::draw_world,
  world::draw_current_chunk_border,
};
use block_placement::block_placement_system;
use delta_time::{DeltaTime, init_delta_time};
use cursor_lock::{insert_lock_state, update_cursor_lock_state, lock_cursor_now};
use control_flow::{exit_on_esc, insert_control_flow_unique, SetControlFlow};

fn startup() -> Workload {
  (
    load_settings,
    load_prefabs,
    init_simple_box_buffers,
    insert_lock_state,
    lock_cursor_now,
    init_input,
    init_game_world,
    spawn_player,
    insert_control_flow_unique,
    init_delta_time,
  ).into_workload()
}
fn update() -> Workload {
  (
    process_inputs,
    update_controllers,
    generate_move_events,
    update_loaded_world_around_player,
    update_raycasts,
    block_placement_system,
    apply_queued_blocks,
    update_cursor_lock_state,
    compute_cameras,
    exit_on_esc,
  ).into_workload()
}
fn render() -> Workload {
  (
    clear_background,
    draw_world,
    draw_current_chunk_border,
    render_selection_box,
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
    #[allow(clippy::collapsible_match, clippy::single_match)]
    match event {
      Event::WindowEvent { event, .. } => match event {
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

        //After frame end
        world.run_workload(after_frame_end).unwrap();

        //Process control flow changes
        if let Some(flow) = world.borrow::<UniqueView<SetControlFlow>>().unwrap().0 {
          *control_flow = flow;
        }
      },
      _ => (),
    };
  });
}
