use shipyard::{World, Workload, IntoWorkload};
use std::{thread, time::Duration};

pub(crate) mod config;
pub(crate) mod server;
pub(crate) mod client;
pub(crate) mod transform;

use config::read_config;
use server::{bind_server, update_server};

fn initialize() -> Workload {
  (
    read_config,
    bind_server,
  ).into_workload()
}

fn update() -> Workload {
  (
    update_server,
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
