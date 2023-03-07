use shipyard::{World, Workload, IntoWorkload};
use std::{thread, time::Duration};

pub(crate) mod util;
pub(crate) mod config;
pub(crate) mod server;
pub(crate) mod client;
pub(crate) mod world;
pub(crate) mod auth;

use config::read_config;
use server::{bind_server, update_server, update_server_events};
use auth::authenticate_players;
use world::{update_world, init_world};

fn initialize() -> Workload {
  (
    read_config,
    bind_server,
    init_world,
  ).into_workload()
}

fn update() -> Workload {
  (
    update_server,
    update_server_events,
    authenticate_players,
    update_world,
  ).into_workload()
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
