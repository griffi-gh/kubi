use shipyard::{Component, EntityId, Unique, Workload, AllStoragesView};
use hashbrown::HashMap;
use nohash_hasher::BuildNoHashHasher;
use std::net::SocketAddr;
use kubi_shared::networking::client::ClientId;

#[derive(Component, Clone, Copy)]
pub struct ClientAddress(pub SocketAddr);

#[derive(Unique)]
pub struct ClientIdMap(pub HashMap<ClientId, EntityId, BuildNoHashHasher<ClientId>>);
impl ClientIdMap {
  pub fn new() -> Self {
    Self(HashMap::with_hasher(BuildNoHashHasher::default()))
  }
}
impl Default for ClientIdMap {
  fn default() -> Self {
    Self::new()
  }
}

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
