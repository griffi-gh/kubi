use glam::Vec3;
use hashbrown::HashMap;
use nohash_hasher::BuildNoHashHasher;
use shipyard::{UniqueViewMut, View, IntoIter, Unique, EntityId, AllStoragesView};
use uflow::{SendMode, client::Event as ClientEvent};
use kubi_shared::networking::{
  messages::{ClientToServerMessage, ServerToClientMessage, S_PLAYER_POSITION_CHANGED},
  channels::CHANNEL_MOVE, 
  client::{ClientId, ClientIdMap},
};
use crate::events::player_actions::PlayerActionEvent;
use super::{UdpClient, NetworkEvent};

pub fn init_client_map(
  storages: AllStoragesView,
) {
  storages.add_unique(ClientIdMap::new());
}

pub fn add_net_player() {
  //TODO
}

pub fn send_player_movement_events(
  actions: View<PlayerActionEvent>,
  mut client: UniqueViewMut<UdpClient>,
) {
  for event in actions.iter() {
    let PlayerActionEvent::PositionChanged { position, direction } = event else {
      continue
    };
    client.0.send(
      postcard::to_allocvec(&ClientToServerMessage::PositionChanged {
        position: *position,
        velocity: Vec3::ZERO,
        direction: *direction
      }).unwrap().into_boxed_slice(), 
      CHANNEL_MOVE,
      SendMode::TimeSensitive
    );
  }
}

pub fn receive_player_movement_events(
  network_events: View<NetworkEvent>,
) {
  for event in network_events.iter() {
    let ClientEvent::Receive(data) = &event.0 else {
      continue
    };
    if !event.is_message_of_type::<S_PLAYER_POSITION_CHANGED>() {
      continue
    }
    let Ok(parsed_message) = postcard::from_bytes(data) else {
      log::error!("Malformed message");
      continue
    };
    let ServerToClientMessage::PlayerPositionChanged {
      client_id, position, direction
    } = parsed_message else { unreachable!() };
    //TODO apply position to local player
  }
}


pub fn receive_connected_players(
  network_events: View<NetworkEvent>,
) {

}
