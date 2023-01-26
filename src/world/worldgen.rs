use glam::IVec3;
use super::{
  chunk::{BlockData, CHUNK_SIZE},
  block::Block
};

pub fn generate_world(position: IVec3, _seed: u32) -> BlockData {
  let mut blocks = Box::new([[[Block::Air; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]);
  //TODO actual world generation
  if position.y == -1 {
    for x in 0..CHUNK_SIZE {
      for z in 0..CHUNK_SIZE {
        blocks[x][0][z] = Block::Grass;
      }
    }
  }
  blocks
}
