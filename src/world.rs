use glam::IVec2;
use hashbrown::HashMap;

pub mod chunk;
pub mod block;

use chunk::Chunk;

pub struct World {
  pub chunks: HashMap<IVec2, Chunk>
}
