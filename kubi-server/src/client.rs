use shipyard::{Component, EntityId, Unique, AllStoragesView, UniqueView, NonSendSync, View, Get};
use hashbrown::HashMap;
use std::net::SocketAddr;
use uflow::server::Event as ServerEvent;
use kubi_shared::networking::{
  client::{ClientIdMap, Client}, 
  messages::{ClientToServerMessage, C_POSITION_CHANGED}
};
use crate::server::{ServerEvents, IsMessageOfType, UdpServer};

#[derive(Component, Clone, Copy)]
pub struct ClientAddress(pub SocketAddr);

#[derive(Unique, Default)]
pub struct ClientAddressMap(pub HashMap<SocketAddr, EntityId>);
impl ClientAddressMap {
  pub fn new() -> Self { Self::default() }
}

pub fn init_client_maps(
  storages: AllStoragesView
) {
  storages.add_unique(ClientIdMap::new());
  storages.add_unique(ClientAddressMap::new());
}

pub fn sync_client_positions(
  server: NonSendSync<UniqueView<UdpServer>>,
  events: UniqueView<ServerEvents>,
  addr_map: UniqueView<ClientAddressMap>,
  clients: View<Client>,
) {
  for event in &events.0 {
    let ServerEvent::Receive(client_addr, data) = event else{
      continue
    };
    if !event.is_message_of_type::<C_POSITION_CHANGED>() {
      continue
    }
    let Some(client) = server.0.client(client_addr) else {
      log::error!("Client doesn't exist");
      continue
    };
    let Some(&entity_id) = addr_map.0.get(client_addr) else {
      log::error!("Client not authenticated");
      continue
    };
    let Ok(&Client(client_id)) = (&clients).get(entity_id) else {
      log::error!("Entity ID is invalid");
      continue
    };
    let Ok(parsed_message) = postcard::from_bytes(data) else {
      log::error!("Malformed message");
      continue
    };
    let ClientToServerMessage::PositionChanged { position, velocity, direction } = parsed_message else {
      unreachable!()
    };

    
  }
}
