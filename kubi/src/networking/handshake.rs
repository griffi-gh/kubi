use glam::Mat4;
use shipyard::{UniqueViewMut, View, IntoIter, AllStoragesViewMut, ViewMut, IntoWithId};
use uflow::{client::Event as ClientEvent, SendMode};
use kubi_shared::{
  networking::{
    messages::{ClientToServerMessage, ServerToClientMessage, S_SERVER_HELLO},
    state::ClientJoinState, 
    channels::CHANNEL_AUTH, 
    client::{Username, Client},
  }, 
  transform::Transform, entity::Health
};
use crate::player::MainPlayer;
use super::{UdpClient, NetworkEvent, player::add_net_player};

pub fn set_client_join_state_to_connected(
  mut join_state: UniqueViewMut<ClientJoinState>
) {
  log::info!("Setting ClientJoinState");
  *join_state = ClientJoinState::Connected;
}

pub fn say_hello(
  mut client: UniqueViewMut<UdpClient>,
  main_player: View<MainPlayer>,
  username: View<Username>
) {
  let username = (&main_player, &username).iter().next().unwrap().1.0.clone();
  let password = None;
  log::info!("Authenticating");
  client.0.send(
    postcard::to_allocvec(
      &ClientToServerMessage::ClientHello { username, password }
    ).unwrap().into_boxed_slice(),
    CHANNEL_AUTH,
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
    if !event.is_message_of_type::<S_SERVER_HELLO>() {
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
  {
    let entity = (&storages.borrow::<View<MainPlayer>>().unwrap()).iter().with_id().next().unwrap().0;
    storages.add_component(entity, Client(init.user.client_id));
  }

  //Modify main player
  {
    for (entity, (_, mut username, mut transform, mut health)) in (
      &storages.borrow::<View<MainPlayer>>().unwrap(),
      &mut storages.borrow::<ViewMut<Username>>().unwrap(),
      &mut storages.borrow::<ViewMut<Transform>>().unwrap(),
      &mut storages.borrow::<ViewMut<Health>>().unwrap(),
    ).iter().with_id() {
      username.0 = init.user.username.clone();
      transform.0 = Mat4::from_rotation_translation(init.user.direction, init.user.position);
      *health = init.user.health;
    }
  }

  //Init players
  for init_data in init.users {
    add_net_player(&mut storages, init_data);
  }

  // Set state to connected
  let mut join_state = storages.borrow::<UniqueViewMut<ClientJoinState>>().unwrap();
  *join_state = ClientJoinState::Joined;

  log::info!("Joined the server!");
}
