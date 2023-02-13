use shipyard::{AllStoragesView, Unique, UniqueView, UniqueViewMut};
use kubi_udp::server::{Server, ServerConfig};
use kubi_shared::networking::messages::{ClientToServerMessage, ServerToClientMessage};
use std::time::Duration;
use crate::config::ConfigTable;

#[derive(Unique)]
#[repr(transparent)]
pub struct UdpServer(Server<ServerToClientMessage, ClientToServerMessage>);

pub fn bind_server(
  storages: AllStoragesView,
) {
  log::info!("Creating server...");
  let config = storages.borrow::<UniqueView<ConfigTable>>().unwrap();
  let server: Server<ServerToClientMessage, ClientToServerMessage> = Server::bind(
    config.server.address, 
    ServerConfig { 
      max_clients: config.server.max_clients,
      client_timeout: Duration::from_millis(config.server.timeout_ms),
      ..Default::default()
    }
  ).unwrap();
  storages.add_unique(UdpServer(server));
}

pub fn update_server(
  mut server: UniqueViewMut<UdpServer>
) {
  if let Err(error) = server.0.update() {
    log::error!("Server error: {error:?}")
  }
}
