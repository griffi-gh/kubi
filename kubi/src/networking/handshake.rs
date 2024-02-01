use shipyard::{UniqueViewMut, View, IntoIter, AllStoragesViewMut};
use uflow::{client::Event as ClientEvent, SendMode};
use kubi_shared::networking::{
  messages::{ClientToServerMessage, ServerToClientMessage, ServerToClientMessageType},
  state::ClientJoinState,
  channels::Channel,
};
use crate::player::{spawn_local_player_multiplayer, spawn_remote_player_multiplayer};
use super::{UdpClient, NetworkEvent};

pub fn set_client_join_state_to_connected(
  mut join_state: UniqueViewMut<ClientJoinState>
) {
  log::info!("Setting ClientJoinState");
  *join_state = ClientJoinState::Connected;
}

pub fn say_hello(
  mut client: UniqueViewMut<UdpClient>,
) {
  let username = "XxX-FishFucker-69420-XxX".into(); 
  let password = None;
  log::info!("Authenticating");
  client.0.send(
    postcard::to_allocvec(
      &ClientToServerMessage::ClientHello { username, password }
    ).unwrap().into_boxed_slice(),
    Channel::Auth as usize,
    SendMode::Reliable
  );
}

pub fn check_server_hello_response(
  mut storages: AllStoragesViewMut,
) {
  //Check if we got the message and extract the init data from it
  let Some(init) = storages.borrow::<View<NetworkEvent>>().unwrap().iter().find_map(|event| {
    let ClientEvent::Receive(data) = &event.0 else {
      return None
    };
    if !event.is_message_of_type::<{ServerToClientMessageType::ServerHello as u8}>() {
      return None
    }
    let Ok(parsed_message) = postcard::from_bytes(data) else {
      log::error!("Malformed message");
      return None
    };
    let ServerToClientMessage::ServerHello { init } = parsed_message else {
      unreachable!()
    };
    Some(init)
  }) else { return };

  //  struct ClientInitData {
  //    client_id: ClientId,
  //    username: String,
  //    position: Vec3,
  //    velocity: Vec3,
  //    direction: Quat,
  //    health: Health,
  //  }
  
  //Add components to main player
  spawn_local_player_multiplayer(&mut storages, init.user);

  //Init players
  for init_data in init.users {
    spawn_remote_player_multiplayer(&mut storages, init_data);
  }

  // Set state to connected
  let mut join_state = storages.borrow::<UniqueViewMut<ClientJoinState>>().unwrap();
  *join_state = ClientJoinState::Joined;

  log::info!("Joined the server!");
}
