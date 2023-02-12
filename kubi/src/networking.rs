use shipyard::{Unique, AllStoragesView, UniqueView, UniqueViewMut, Workload, IntoWorkload, WorkloadModificator, EntitiesView, EntitiesViewMut, Component, ViewMut, SystemModificator};
use std::net::SocketAddr;
use kubi_udp::client::{Client, ClientConfig, ClientEvent};
use kubi_shared::networking::{
  messages::{ClientToServerMessage, ServerToClientMessage},
  state::ClientJoinState
};

use crate::events::EventComponent;

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
  let address = storages.borrow::<UniqueView<ServerAddress>>().unwrap();
  storages.add_unique(UdpClient(Client::new(
    address.0, 
    ClientConfig::default()
  ).unwrap()));
  storages.add_unique(ClientJoinState::Disconnected);
}

pub fn connect_client(
  mut client: UniqueViewMut<UdpClient>
) {
  if !client.0.has_not_made_connection_attempts() {
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
    connect_client,
    update_client,
    insert_client_events,
  ).into_workload()
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
