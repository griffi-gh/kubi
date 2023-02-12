use shipyard::{Component, EntityId};
use hashbrown::HashMap;
use nohash_hasher::BuildNoHashHasher;
use kubi_udp::{ClientId, ClientIdRepr};

#[derive(Component)]
pub struct Client(ClientId);


// disconnected => connect => join => load => ingame
#[derive(Component)]
pub enum ClientJoinState {
  /// Client has connected to the game, but haven't authenticated yet
  Connected,
  /// Client has joined the game, but haven't loaded the world yet
  Joined,
  /// Client is currently ingame 
  InGame,
}

pub struct ClientMap(HashMap<ClientId, EntityId, BuildNoHashHasher<ClientIdRepr>>);
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
