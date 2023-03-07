use glam::IVec3;
use hashbrown::HashSet;
use nohash_hasher::BuildNoHashHasher;
use kubi_shared::chunk::BlockData;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChunkState {
  Nothing,
  Loading,
  Loaded,
}

pub struct Chunk {
  pub position: IVec3,
  pub state: ChunkState,
  pub blocks: Option<BlockData>,
  pub subscriptions: HashSet<ClientId, BuildNoHashHasher<ClientIdRepr>>,
}
impl Chunk {
  pub fn new(position: IVec3) -> Self {
    Self {
      position,
      state: ChunkState::Nothing,
      blocks: None,
      subscriptions: HashSet::with_hasher(BuildNoHashHasher::default()),
    }
  }
}
