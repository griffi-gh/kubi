#![allow(clippy::too_many_arguments)] // allowed because systems often need a lot of arguments

use shipyard::{
  World, Workload, IntoWorkload, 
  UniqueView, UniqueViewMut, 
  NonSendSync, WorkloadModificator, 
  SystemModificator
};
use winit::{
  event_loop::{EventLoop, ControlFlow},
  event::{Event, WindowEvent}
};
use glam::vec3;
use std::time::Instant;

pub(crate) use kubi_shared::transform;
pub(crate) mod rendering;
pub(crate) mod world;
pub(crate) mod player;
pub(crate) mod assets;
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
pub(crate) mod networking;
pub(crate) mod init;
pub(crate) mod color;
pub(crate) mod loading_screen;
pub(crate) mod connecting_screen;
pub(crate) mod fixed_timestamp;
pub(crate) mod filesystem;

use world::{
  init_game_world,
  loading::update_loaded_world_around_player,
  raycast::update_raycasts,
  queue::apply_queued_blocks,
  tasks::ChunkTaskManager,
};
use player::{spawn_player, MainPlayer};
use assets::load_prefabs;
use settings::{load_settings, GameSettings};
use camera::compute_cameras;
use events::{
  clear_events,
  process_glutin_events,
  initial_resize_event,
  player_actions::generate_move_events,
};
use input::{init_input, process_inputs};
use fly_controller::update_controllers;
use rendering::{
  Renderer,
  RenderTarget,
  BackgroundColor,
  world::{draw_world, draw_current_chunk_border},
  selection_box::render_selection_box,
  entities::render_entities,
  renderer_finish_init,
};
use block_placement::update_block_placement;
use delta_time::{DeltaTime, init_delta_time};
use cursor_lock::{insert_lock_state, update_cursor_lock_state, lock_cursor_now};
use control_flow::{exit_on_esc, insert_control_flow_unique, SetControlFlow};
use state::{is_ingame, is_ingame_or_loading, is_loading, init_state, update_state, is_connecting};
use networking::{update_networking, update_networking_late, is_multiplayer, disconnect_on_exit, is_singleplayer};
use init::initialize_from_args;
use loading_screen::update_loading_screen;
use connecting_screen::switch_to_loading_if_connected;
use fixed_timestamp::init_fixed_timestamp_storage;

/// stuff required to init the renderer and other basic systems
fn pre_startup() -> Workload {
  (
    load_settings,
  ).into_sequential_workload()
}

fn startup() -> Workload {
  (
    renderer_finish_init,
    init_fixed_timestamp_storage,
    initial_resize_event,
    load_prefabs,
    insert_lock_state,
    init_state,
    initialize_from_args,
    lock_cursor_now,
    init_input,
    insert_control_flow_unique,
    init_delta_time,
  ).into_sequential_workload()
}

fn update() -> Workload {
  (
    update_cursor_lock_state,
    process_inputs,
    (
      init_game_world.run_if_missing_unique::<ChunkTaskManager>(),
      (
        spawn_player.run_if_storage_empty::<MainPlayer>(),
      ).into_sequential_workload().run_if(is_singleplayer),
    ).into_sequential_workload().run_if(is_ingame_or_loading),
    update_networking().run_if(is_multiplayer),
    (
      switch_to_loading_if_connected
    ).into_sequential_workload().run_if(is_connecting),
    (
      update_loading_screen,
    ).into_sequential_workload().run_if(is_loading),
    (
      update_loaded_world_around_player,
    ).into_sequential_workload().run_if(is_ingame_or_loading),
    (
      update_controllers,
      generate_move_events,
      update_raycasts,
      update_block_placement,
      apply_queued_blocks,
    ).into_sequential_workload().run_if(is_ingame),
    update_networking_late.run_if(is_multiplayer),
    compute_cameras,
    update_state,
    exit_on_esc,
    disconnect_on_exit.run_if(is_multiplayer),
  ).into_sequential_workload()
}

fn render() -> Workload {
  (
    (
      draw_world,
      draw_current_chunk_border,
      render_selection_box,
      render_entities,
    ).into_sequential_workload().run_if(is_ingame),
  ).into_sequential_workload()
}

fn after_frame_end() -> Workload {
  (
    clear_events,
  ).into_sequential_workload()
}

#[cfg(all(windows, not(debug_assertions)))]
fn attach_console() {
  use winapi::um::wincon::{AttachConsole, ATTACH_PARENT_PROCESS};
  unsafe { AttachConsole(ATTACH_PARENT_PROCESS); }
}

#[cfg(target_os = "android")]
#[no_mangle]
pub fn android_main(app: AndroidApp) {
  kubi_main()
}

pub fn kubi_main() {
  //Attach console on release builds on windows
  #[cfg(all(windows, not(debug_assertions)))] attach_console();

  //Print version
  println!("{:─^54}", format!("[ ▄▀ Kubi client v. {} ]", env!("CARGO_PKG_VERSION")));
  
  //Init env_logger
  kubi_logging::init();

  //Create a shipyard world
  let mut world = World::new();
  
  //Register workloads
  world.add_workload(pre_startup);
  world.add_workload(startup);
  world.add_workload(update);
  world.add_workload(render);
  world.add_workload(after_frame_end);
  
  //Run pre-startup procedure
  world.run_workload(pre_startup).unwrap();
  
  //Create event loop
  let event_loop = EventLoop::new();

  //Initialize renderer
  {
    let settings = world.borrow::<UniqueView<GameSettings>>().unwrap();
    world.add_unique_non_send_sync(Renderer::init_blocking(&event_loop, &settings));
  }
  world.add_unique(BackgroundColor(vec3(0.5, 0.5, 1.)));

  //Save _visualizer.json
  #[cfg(feature = "generate_visualizer_data")]
  std::fs::write(
    "_visualizer.json",
    serde_json::to_string(&world.workloads_info()).unwrap(),
  ).unwrap();

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
        WindowEvent::Resized(new_size) => {
          //this can be in a system but I don't care
          world.borrow::<NonSendSync<UniqueViewMut<Renderer>>>().unwrap().resize(new_size);
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
        
        //Run update workflows
        world.run_workload(update).unwrap();
        
        //Start rendering (maybe use custom views for this?)
        let target = {
          let renderer = world.borrow::<NonSendSync<UniqueView<Renderer>>>().unwrap();
          renderer.begin()
        };
        world.add_unique_non_send_sync(target);

        //Run render workflow
        world.run_workload(render).unwrap();

        //Finish rendering
        {
          let target = world.remove_unique::<RenderTarget>().unwrap();
          let renderer = world.borrow::<NonSendSync<UniqueView<Renderer>>>().unwrap();
          renderer.end(target);
        }

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
