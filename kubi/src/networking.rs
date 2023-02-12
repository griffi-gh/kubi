use shipyard::{Unique, AllStoragesView, UniqueView, UniqueViewMut, Workload, IntoWorkload, WorkloadModificator};
use std::net::SocketAddr;
use kubi_udp::client::{Client, ClientConfig};
use kubi_shared::networking::{
  messages::{ClientToServerMessage, ServerToClientMessage},
  state::ClientJoinState
};

#[derive(Unique, Clone, Copy, PartialEq, Eq)]
pub enum GameType {
  Singleplayer,
  Muliplayer
}

#[derive(Unique, Clone, Copy, PartialEq, Eq)]
pub struct ServerAddress(pub SocketAddr);

#[derive(Unique)]
pub struct UdpClient(pub Client<ClientToServerMessage, ServerToClientMessage>);

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

pub fn client_connect(
  mut client: UniqueViewMut<UdpClient>
) {
  client.0.connect().unwrap();
}

pub fn update_client_and_get_events(
  mut client: UniqueViewMut<UdpClient>,
) {
  client.0.update().unwrap();
  for event in client.0.process_events() {
    todo!()
  }
}

pub fn init_networking() -> Workload {
  (
    create_client,
    client_connect,
  ).into_workload().run_if(is_multiplayer)
}

pub fn update_networking() -> Workload {
  (
    update_client_and_get_events,
  ).into_workload().run_if(is_multiplayer)
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
