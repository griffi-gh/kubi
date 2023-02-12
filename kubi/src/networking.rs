use shipyard::{Unique, AllStoragesView, UniqueView, UniqueViewMut, Workload, IntoWorkload, EntitiesViewMut, Component, ViewMut, SystemModificator};
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

pub fn create_client(
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

pub fn connect_client_if_needed(
  mut client: UniqueViewMut<UdpClient>
) {
  //NOTE: this used to be a condition function
  //but that caused some issues for no reason
  if client.0.has_not_made_connection_attempts() {
    log::info!("Connect called");
    client.0.connect().unwrap();
  }
}

pub fn update_client(
  mut client: UniqueViewMut<UdpClient>,
) {
  client.0.update().unwrap();
}

pub fn insert_client_events(
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

pub fn update_networking() -> Workload {
  (
    create_client.run_if_missing_unique::<UdpClient>(),
    connect_client_if_needed,
    update_client,
    insert_client_events,
  ).into_workload()
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
