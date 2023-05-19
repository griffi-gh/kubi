use shipyard::{Component, Unique, EntityId};
use hashbrown::HashMap;
use nohash_hasher::BuildNoHashHasher;

pub type ClientId = u16;

#[derive(Component, Clone, Copy, Debug)]
pub struct Client(pub ClientId);

#[derive(Unique)]
pub struct ClientIdMap(pub HashMap<ClientId, EntityId, BuildNoHashHasher<ClientId>>);
impl ClientIdMap {
  pub fn new() -> Self {
    Self(HashMap::with_capacity_and_hasher(16, BuildNoHashHasher::default()))
  }
}
impl Default for ClientIdMap {
  fn default() -> Self {
    Self::new()
  }
}
