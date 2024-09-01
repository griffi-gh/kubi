//TODO move lints to workspace Cargo.toml
#![allow(
  clippy::too_many_arguments, // allowed because systems often need a lot of argumentss
  clippy::enum_variant_names,
  clippy::type_complexity
)]
#![forbid(
  static_mut_refs,
  unsafe_op_in_unsafe_fn,
  rust_2024_compatibility,
)]

use shipyard::{
  World, Workload, IntoWorkload,
  UniqueView, UniqueViewMut,
  WorkloadModificator,
  SystemModificator
};
use winit::{
  event_loop::{EventLoop, ControlFlow},
  event::{Event, WindowEvent}
};
use glam::vec3;
use std::time::Instant;

pub(crate) use kubi_shared::transform;

mod ui;
pub(crate) use ui::{
  loading_screen,
  connecting_screen,
  chat_ui,
  crosshair_ui,
  settings_ui,
};
pub(crate) mod rendering;
pub(crate) mod world;
pub(crate) mod player;
pub(crate) mod prefabs;
pub(crate) mod settings;
pub(crate) mod camera;
pub(crate) mod events;
pub(crate) mod input;
pub(crate) mod player_controller;
pub(crate) mod block_placement;
pub(crate) mod delta_time;
pub(crate) mod cursor_lock;
pub(crate) mod control_flow;
pub(crate) mod state;
pub(crate) mod hui_integration;
pub(crate) mod networking;
pub(crate) mod init;
pub(crate) mod color;
pub(crate) mod fixed_timestamp;
pub(crate) mod filesystem;
pub(crate) mod client_physics;
pub(crate) mod chat;

use world::{
  init_game_world,
  loading::update_loaded_world_around_player,
  raycast::update_raycasts,
  queue::apply_queued_blocks,
  tasks::ChunkTaskManager,
};
use player::{spawn_player, MainPlayer};
use prefabs::load_prefabs;
use settings::{load_settings, GameSettings};
use camera::compute_cameras;
use events::{clear_events, process_winit_events, player_actions::generate_move_events};
use input::{init_input, process_inputs};
use player_controller::{debug_switch_ctl_type, update_player_controllers};
use rendering::{BackgroundColor, Renderer, init_rendering, render_master, update_rendering_early, update_rendering_late};
use block_placement::update_block_placement;
use delta_time::{DeltaTime, init_delta_time};
use cursor_lock::{debug_toggle_lock, insert_lock_state, lock_cursor_now, update_cursor_lock_state};
use control_flow::{exit_on_esc, insert_control_flow_unique, RequestExit};
use state::{is_ingame, is_ingame_or_loading, is_loading, init_state, update_state, is_connecting};
use networking::{update_networking, update_networking_late, is_multiplayer, disconnect_on_exit, is_singleplayer};
use init::initialize_from_args;
use hui_integration::{kubi_ui_begin, /*kubi_ui_draw,*/ kubi_ui_end, kubi_ui_init};
use loading_screen::update_loading_screen;
use connecting_screen::update_connecting_screen;
use fixed_timestamp::init_fixed_timestamp_storage;
use filesystem::AssetManager;
use client_physics::{init_client_physics, update_client_physics_late};
use chat_ui::render_chat;
use chat::init_chat_manager;
use crosshair_ui::{init_crosshair_image, draw_crosshair};
use settings_ui::render_settings_ui;
use hui_integration::hui_process_winit_events;

/// stuff required to init the renderer and other basic systems
fn pre_startup() -> Workload {
  (
    load_settings,
  ).into_sequential_workload()
}

fn startup() -> Workload {
  (
    init_fixed_timestamp_storage,
    kubi_ui_init,
    load_prefabs,
    init_rendering,
    insert_lock_state,
    init_state,
    initialize_from_args,
    lock_cursor_now,
    init_input,
    insert_control_flow_unique,
    init_delta_time,
    init_client_physics,
    init_chat_manager,
    init_crosshair_image,
  ).into_sequential_workload()
}

fn update() -> Workload {
  (
    update_rendering_early,
    debug_toggle_lock,
    update_cursor_lock_state,
    process_inputs,
    kubi_ui_begin,
    (
      init_game_world.run_if_missing_unique::<ChunkTaskManager>(),
      (
        spawn_player.run_if_storage_empty::<MainPlayer>(),
      ).into_sequential_workload().run_if(is_singleplayer),
    ).into_sequential_workload().run_if(is_ingame_or_loading),
    update_networking().run_if(is_multiplayer),
    (
      update_connecting_screen,
    ).into_sequential_workload().run_if(is_connecting),
    (
      update_loading_screen,
    ).into_sequential_workload().run_if(is_loading),
    (
      update_loaded_world_around_player,
    ).into_sequential_workload().run_if(is_ingame_or_loading),
    (
      debug_switch_ctl_type,
      update_player_controllers,
      update_client_physics_late,
      generate_move_events,
      update_raycasts,
      update_block_placement,
      apply_queued_blocks,
      //UI:
      render_chat,
      draw_crosshair,
      render_settings_ui,
    ).into_sequential_workload().run_if(is_ingame),
    update_networking_late.run_if(is_multiplayer),
    compute_cameras,
    kubi_ui_end,
    update_state,
    exit_on_esc,
    disconnect_on_exit.run_if(is_multiplayer),
    update_rendering_late,
  ).into_sequential_workload()
}

// fn render() -> Workload {
//   (
//     clear_background,
//     (
//       draw_world,
//       draw_current_chunk_border,
//       render_selection_box,
//       render_entities,
//       draw_world_trans,
//       render_submerged_view,
//     ).into_sequential_workload().run_if(is_ingame),
//     kubi_ui_draw,
//   ).into_sequential_workload()
// }

fn after_render() -> Workload {
  (
    clear_events,
  ).into_sequential_workload()
}

#[cfg(all(windows, not(debug_assertions)))]
fn attach_console() {
  use winapi::um::wincon::{AttachConsole, ATTACH_PARENT_PROCESS};
  unsafe { AttachConsole(ATTACH_PARENT_PROCESS); }
}

#[unsafe(no_mangle)]
#[cfg(target_os = "android")]
pub fn android_main(app: android_activity::AndroidApp) {
  use android_activity::WindowManagerFlags;
  app.set_window_flags(WindowManagerFlags::FULLSCREEN, WindowManagerFlags::empty());
  kubi_main(app)
}

#[unsafe(no_mangle)]
pub fn kubi_main(
  #[cfg(target_os = "android")]
  app: android_activity::AndroidApp
) {
  //Attach console on release builds on windows
  #[cfg(all(windows, not(debug_assertions)))]
  attach_console();

  //Print version
  println!("{:─^54}", format!("[ ▄▀ Kubi client v. {} ]", env!("CARGO_PKG_VERSION")));

  //Init env_logger
  kubi_logging::init();

  //Create a shipyard world
  let mut world = World::new();

  //Init assman
  world.add_unique(AssetManager {
    #[cfg(target_os = "android")]
    app: app.clone()
  });

  //Register workloads
  world.add_workload(pre_startup);
  world.add_workload(startup);
  world.add_workload(update);
  //world.add_workload(render);
  world.add_workload(after_render);

  //Save _visualizer.json
  #[cfg(feature = "generate_visualizer_data")]
  std::fs::write(
    "_visualizer.json",
    serde_json::to_string(&world.workloads_info()).unwrap(),
  ).unwrap();

  //Run pre-startup procedure
  world.run_workload(pre_startup).unwrap();

  //Create event loop
  let event_loop ={
    #[cfg(not(target_os = "android"))] { EventLoop::new().unwrap() }
    #[cfg(target_os = "android")] {
      use winit::{
        platform::android::EventLoopBuilderExtAndroid,
        event_loop::EventLoopBuilder
      };
      EventLoopBuilder::new().with_android_app(app).build().unwrap()
    }
  };

  //Run the event loop
  let mut last_update = Instant::now();
  let mut ready = false;
  event_loop.run(move |event, window_target| {
    //Wait for the window to become active (required for android)
    if !ready {
      if Event::Resumed != event {
        window_target.set_control_flow(ControlFlow::Wait);
        return
      }

      //Initialize renderer
      {
        let settings = world.borrow::<UniqueView<GameSettings>>().unwrap();
        world.add_unique_non_send_sync(Renderer::init(window_target, &settings));
      }
      world.add_unique(BackgroundColor(vec3(0.21, 0.21, 1.)));

      //Run startup systems
      world.run_workload(startup).unwrap();

      ready = true;
    }

    window_target.set_control_flow(ControlFlow::Poll);

    world.run_with_data(hui_process_winit_events, &event);
    process_winit_events(&mut world, &event);

    #[allow(clippy::collapsible_match, clippy::single_match)]
    match event {
      #[cfg(target_os = "android")]
      Event::Suspended => {
        window_target.exit();
      }

      Event::WindowEvent { event, .. } => match event {
        WindowEvent::CloseRequested => {
          log::info!("exit requested");
          window_target.exit();
        },
        _ => (),
      },

      Event::AboutToWait => {
        //Update delta time (maybe move this into a system?)
        {
          let mut dt_view = world.borrow::<UniqueViewMut<DeltaTime>>().unwrap();
          let now = Instant::now();
          dt_view.0 = now - last_update;
          last_update = now;
        }

        //Run update workflows
        world.run_workload(update).unwrap();

        world.run(render_master);

        //Start rendering (maybe use custom views for this?)
        // let target = {
        //   let renderer = world.borrow::<UniqueView<Renderer>>().unwrap();
        //   renderer.display.draw()
        // };
        // world.add_unique_non_send_sync(RenderTarget(target));

        //Run render workflow
        //world.run_workload(render).unwrap();

        //Finish rendering
        // let target = world.remove_unique::<RenderTarget>().unwrap();
        // target.0.finish().unwrap();

        //After frame end
        world.run_workload(after_render).unwrap();

        //Process control flow changes
        if world.borrow::<UniqueView<RequestExit>>().unwrap().0 {
          window_target.exit();
        }
      },
      _ => (),
    };
  }).unwrap();
}
