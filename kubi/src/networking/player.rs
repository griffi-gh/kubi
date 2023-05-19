use glam::{Vec3, Mat4};
use shipyard::{UniqueViewMut, View, IntoIter, AllStoragesView, AllStoragesViewMut, UniqueView, ViewMut, Get};
use uflow::{SendMode, client::Event as ClientEvent};
use kubi_shared::{
  player::{Player, PlayerHolding},
  entity::Entity,
  transform::Transform,
  networking::{
    messages::{ClientToServerMessage, ServerToClientMessage, S_PLAYER_POSITION_CHANGED, ClientInitData},
    channels::CHANNEL_MOVE, 
    client::{ClientIdMap, Username, Client},
  }, 
};
use crate::events::player_actions::PlayerActionEvent;
use super::{UdpClient, NetworkEvent};

pub fn init_client_map(
  storages: AllStoragesView,
) {
  storages.add_unique(ClientIdMap::new());
}

pub fn add_net_player(
  storages: &mut AllStoragesViewMut,
  init: ClientInitData
) {
  //  struct ClientInitData {
  //    client_id: ClientId,
  //    username: String,
  //    position: Vec3,
  //    velocity: Vec3,
  //    direction: Quat,
  //    health: Health,
  //  }
  
  //Spawn player locally
  let entity_id = storages.add_entity((
    Username(init.username),
    Client(init.client_id),
    Player,
    Entity,
    init.health,
    Transform(Mat4::from_rotation_translation(init.direction, init.position)),
    PlayerHolding::default(),
  ));

  //Add it to the client id map
  let mut client_id_map = storages.borrow::<UniqueViewMut<ClientIdMap>>().unwrap();
  client_id_map.0.insert(init.client_id, entity_id);
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
  mut transforms: ViewMut<Transform>,
  network_events: View<NetworkEvent>,
  id_map: UniqueView<ClientIdMap>
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

    let Some(&ent_id) = id_map.0.get(&client_id) else {
      log::error!("Not in client-id map");
      continue
    };

    let mut transform = (&mut transforms).get(ent_id)
      .expect("invalid player entity id");

    transform.0 = Mat4::from_rotation_translation(direction, position);
  }
}

pub fn receive_player_connect_events(
  mut storages: AllStoragesViewMut,
) {
  let messages: Vec<ServerToClientMessage> = storages.borrow::<View<NetworkEvent>>().unwrap().iter().filter_map(|event| {
    let ClientEvent::Receive(data) = &event.0 else {
      return None
    };
    if !event.is_message_of_type::<S_PLAYER_POSITION_CHANGED>() {
      return None
    };
    let Ok(parsed_message) = postcard::from_bytes(data) else {
      log::error!("Malformed message");
      return None
    };
    Some(parsed_message)
  }).collect();

  for message in messages {
    let ServerToClientMessage::PlayerConnected { init } = message else { unreachable!() };
    log::info!("player connected: {} (id {})", init.username, init.client_id);
    add_net_player(&mut storages, init);
  }
}
