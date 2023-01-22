use glam::IVec3;
use super::{
  chunk::{BlockData, CHUNK_SIZE},
  block::Block
};

pub fn generate_world(position: IVec3, seed: u32) -> BlockData {
  let mut blocks = Box::new([[[Block::Air; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]);
  blocks[0][0][0] = Block::Stone;
  //TODO actual world generation
  blocks
}
