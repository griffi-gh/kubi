use shipyard::{World, AllStoragesView, Unique, Workload, IntoWorkload, UniqueView, UniqueViewMut};
use kubi_udp::server::{Server, ServerConfig};
use kubi_shared::networking::messages::{ClientToServerMessage, ServerToClientMessage};

#[derive(Unique)]
#[repr(transparent)]
pub struct UdpServer(Server<ServerToClientMessage, ClientToServerMessage>);

fn bind_server(
  storages: AllStoragesView,
) {
  let server: Server<ServerToClientMessage, ClientToServerMessage> = Server::bind(
    "0.0.0.0:1234".parse().unwrap(), 
    ServerConfig::default()
  ).unwrap();
  storages.add_unique(UdpServer(server));
}

fn update_server(
  mut server: UniqueViewMut<UdpServer>
) {
  if let Err(error) = server.0.update() {
    println!("Server error: {error:?}")
  }
}

fn initialize() -> Workload {
  (
    bind_server,
  ).into_workload()
}

fn update() -> Workload {
  (
    update_server,
  ).into_workload()
}

fn main() {
  let world = World::new();
  world.add_workload(initialize);
  world.add_workload(update);
  world.run_workload(initialize).unwrap();
  loop {
    world.run_workload(update).unwrap();
  }
}
