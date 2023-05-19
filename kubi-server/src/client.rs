use glam::Mat4;
use shipyard::{Component, EntityId, Unique, AllStoragesView, UniqueView, NonSendSync, View, ViewMut, Get};
use hashbrown::HashMap;
use std::net::SocketAddr;
use kubi_shared::{
  networking::{
    client::{ClientIdMap, Client}, 
    messages::{ClientToServerMessage, C_POSITION_CHANGED}
  },
  transform::Transform
};
use crate::{
  server::{ServerEvents, UdpServer}, 
  util::check_message_auth
};

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
  mut transforms: ViewMut<Transform>
) {
  for event in &events.0 {
    let Some(message) = check_message_auth::<C_POSITION_CHANGED>(&server, event, &clients, &addr_map) else {
      continue;
    };
    let ClientToServerMessage::PositionChanged { position, velocity: _, direction } = message.message else {
      unreachable!()
    };
    //Apply position to client
    let mut trans = (&mut transforms).get(message.entity_id).unwrap();
    trans.0 = Mat4::from_rotation_translation(direction, position);

  }
}
