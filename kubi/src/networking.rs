use shipyard::{Unique, AllStoragesView, UniqueView, UniqueViewMut, Workload, IntoWorkload, EntitiesViewMut, Component, ViewMut, SystemModificator, View, IntoIter, WorkloadModificator};
use glium::glutin::event_loop::ControlFlow;
use std::net::SocketAddr;
use kubi_udp::client::{Client, ClientConfig, ClientEvent};
use kubi_shared::networking::{
  messages::{ClientToServerMessage, ServerToClientMessage},
  state::ClientJoinState
};
use crate::{events::EventComponent, control_flow::SetControlFlow};

#[derive(Unique, Clone, Copy, PartialEq, Eq)]
pub enum GameType {
  Singleplayer,
  Muliplayer
}

#[derive(Unique, Clone, Copy, PartialEq, Eq)]
pub struct ServerAddress(pub SocketAddr);

#[derive(Unique)]
pub struct UdpClient(pub Client<ClientToServerMessage, ServerToClientMessage>);

#[derive(Component)]
pub struct NetworkEvent(pub ClientEvent<ServerToClientMessage>);

fn create_client(
  storages: AllStoragesView
) {
  log::info!("Creating client");
  let address = storages.borrow::<UniqueView<ServerAddress>>().unwrap();
  storages.add_unique(UdpClient(Client::new(
    address.0, 
    ClientConfig::default()
  ).unwrap()));
  storages.add_unique(ClientJoinState::Disconnected);
}

fn connect_client(
  mut client: UniqueViewMut<UdpClient>
) {
  log::info!("Connect called");
  client.0.connect().unwrap();
}

fn should_connect(
  client: UniqueView<UdpClient>
) -> bool {
  client.0.has_not_made_connection_attempts()
}

fn update_client(
  mut client: UniqueViewMut<UdpClient>,
) {
  client.0.update().unwrap();
}

fn insert_client_events(
  mut client: UniqueViewMut<UdpClient>,
  mut entities: EntitiesViewMut,
  mut events: ViewMut<EventComponent>,
  mut network_events: ViewMut<NetworkEvent>,
) {
  entities.bulk_add_entity((
    &mut events,
    &mut network_events,
  ), client.0.process_events().map(|event| {
    (EventComponent, NetworkEvent(event))
  }));
}

fn set_client_join_state_to_connected(
  mut join_state: UniqueViewMut<ClientJoinState>
) {
  log::info!("Setting ClientJoinState");
  *join_state = ClientJoinState::Connected;
}

fn say_hello(
  client: UniqueViewMut<UdpClient>,
) {
  log::info!("Authenticating");
  client.0.send_message(ClientToServerMessage::ClientHello {
    username: "Sbeve".into(),
    password: None
  }).unwrap();
}

fn check_server_hello_response(
  network_events: View<NetworkEvent>,
  mut join_state: UniqueViewMut<ClientJoinState>
) {
  for event in network_events.iter() {
    if let ClientEvent::MessageReceived(ServerToClientMessage::ServerHello { init }) = &event.0 {
      log::info!("Joined the server!");
      //TODO handle init data
      *join_state = ClientJoinState::Joined;
    }
  }
}

pub fn update_networking() -> Workload {
  (
    create_client.run_if_missing_unique::<UdpClient>(),
    connect_client.run_if(should_connect),
    update_client,
    insert_client_events,
    (
      set_client_join_state_to_connected,
      say_hello,
    ).into_workload().run_if(if_just_connected),
    (
      check_server_hello_response,
    ).into_workload().run_if(is_join_state::<{ClientJoinState::Connected as u8}>)
  ).into_sequential_workload() //Fixes
}

pub fn disconnect_on_exit(
  control_flow: UniqueView<SetControlFlow>,
  mut client: UniqueViewMut<UdpClient>,
) {
  if let Some(ControlFlow::ExitWithCode(_)) = control_flow.0 {
    client.0.set_nonblocking(false).expect("Failed to switch socket to blocking mode");
    if let Err(error) = client.0.disconnect() {
      log::error!("failed to disconnect: {}", error);
    } else {
      log::info!("Client disconnected");
    }
  }
}

// conditions

fn if_just_connected(
  network_events: View<NetworkEvent>,
) -> bool {
  network_events.iter().any(|event| matches!(&event.0, ClientEvent::Connected(_)))
}

fn is_join_state<const STATE: u8>(
  join_state: UniqueView<ClientJoinState>
) -> bool {
  (*join_state as u8) == STATE
}

pub fn is_multiplayer(
  game_type: UniqueView<GameType>
) -> bool {
  *game_type == GameType::Muliplayer
}

pub fn is_singleplayer(
  game_type: UniqueView<GameType>
) -> bool {
  *game_type == GameType::Singleplayer
}
