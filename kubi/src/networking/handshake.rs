use shipyard::{UniqueViewMut, View, IntoIter};
use uflow::{client::Event as ClientEvent, SendMode};
use kubi_shared::networking::{
  messages::{ClientToServerMessage, ServerToClientMessage, S_SERVER_HELLO},
  state::ClientJoinState, 
  channels::CHANNEL_AUTH,
};
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
  log::info!("Authenticating");
  client.0.send(
    postcard::to_allocvec(
      &ClientToServerMessage::ClientHello {
        username: "Sbeve".into(),
        password: None
      }
    ).unwrap().into_boxed_slice(),
    CHANNEL_AUTH,
    SendMode::Reliable
  );
}

pub fn check_server_hello_response(
  network_events: View<NetworkEvent>,
  mut join_state: UniqueViewMut<ClientJoinState>
) {
  for event in network_events.iter() {
    let ClientEvent::Receive(data) = &event.0 else {
      continue
    };
    if !event.is_message_of_type::<S_SERVER_HELLO>() {
      continue
    }
    let Ok(parsed_message) = postcard::from_bytes(data) else {
      log::error!("Malformed message");
      continue
    };
    let ServerToClientMessage::ServerHello { init: _ } = parsed_message else {
      unreachable!()
    };
    //TODO handle init data
    *join_state = ClientJoinState::Joined;
    log::info!("Joined the server!");
    return;
  }
}
