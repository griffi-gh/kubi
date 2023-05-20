use glam::Mat4;
use shipyard::{Component, EntityId, Unique, AllStoragesView, UniqueView, NonSendSync, View, ViewMut, Get, IntoIter};
use hashbrown::HashMap;
use uflow::SendMode;
use std::net::SocketAddr;
use kubi_shared::{
  networking::{
    client::{ClientIdMap, Client}, 
    messages::{ClientToServerMessage, ServerToClientMessage, C_POSITION_CHANGED},
    channels::CHANNEL_MOVE
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
  mut transforms: ViewMut<Transform>,
  addrs: View<ClientAddress>,
) {
  for event in &events.0 {
    let Some(message) = check_message_auth::<C_POSITION_CHANGED>(&server, event, &clients, &addr_map) else {
      continue;
    };
    let ClientToServerMessage::PositionChanged { position, velocity: _, direction } = message.message else {
      unreachable!()
    };

    //log movement (annoying duh)
    log::debug!("dbg: player moved id: {} coords: {} quat: {}", message.client_id, position, direction);

    //Apply position to server-side client
    let mut trans = (&mut transforms).get(message.entity_id).unwrap();
    trans.0 = Mat4::from_rotation_translation(direction, position);

    //Transmit the change to other players
    for (other_client, other_client_address) in (&clients, &addrs).iter() {
      if other_client.0 == message.client_id {
        continue
      }
      let Some(client) = server.0.client(&other_client_address.0) else {
        log::error!("Client with address not found");
        continue
      };
      client.borrow_mut().send(
        postcard::to_allocvec(
          &ServerToClientMessage::PlayerPositionChanged {
            client_id: message.client_id,
            position,
            direction
          }
        ).unwrap().into_boxed_slice(), 
        CHANNEL_MOVE, 
        SendMode::Reliable
      );
    }
  }
}
