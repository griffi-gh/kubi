use shipyard::{World, Workload, IntoWorkload};
use std::{thread, time::Duration};

mod config;
mod server;
mod client;
mod world;
mod auth;

use config::read_config;
use server::{bind_server, update_server, log_server_errors};
use client::init_client_maps;
use auth::authenticate_players;
use world::{update_world, init_world};

fn initialize() -> Workload {
  (
    read_config,
    bind_server,
    init_client_maps,
    init_world,
  ).into_workload()
}

fn update() -> Workload {
  (
    update_server,
    (
      log_server_errors,
      authenticate_players,
      update_world,
    ).into_workload()
  ).into_sequential_workload()
}

fn main() {
  kubi_logging::init();
  let world = World::new();
  world.add_workload(initialize);
  world.add_workload(update);
  world.run_workload(initialize).unwrap();
  log::info!("The server is now running");
  loop {
    world.run_workload(update).unwrap();
    thread::sleep(Duration::from_millis(16));
  }
}
