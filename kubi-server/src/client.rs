use shipyard::{Component, EntityId};
use hashbrown::HashMap;
use nohash_hasher::BuildNoHashHasher;
use kubi_shared::networking::client::ClientId;

#[derive(Component)]
pub struct Client(ClientId);

pub struct ClientMap(HashMap<ClientId, EntityId, BuildNoHashHasher<ClientId>>);
impl ClientMap {
  pub fn new() -> Self {
    Self(HashMap::with_hasher(BuildNoHashHasher::default()))
  }
}
impl Default for ClientMap {
  fn default() -> Self {
    Self::new()
  }
}
