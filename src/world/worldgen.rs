use glam::IVec3;
use super::{
  chunk::{BlockData, CHUNK_SIZE},
  block::Block
};

pub fn generate_world(position: IVec3, _seed: u32) -> BlockData {
  let mut blocks = Box::new([[[Block::Air; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]);
  if position.y == -1 {
    for x in 0..CHUNK_SIZE {
      for z in 0..CHUNK_SIZE {
        blocks[x][0][z] = Block::Grass;
      }
    }
  } else {
    blocks[0][0][0] = Block::Stone;
    blocks[1][0][0] = Block::Stone;
    blocks[0][1][0] = Block::Stone;
    blocks[0][2][0] = Block::Stone;
    blocks[0][0][1] = Block::Stone;
  }
  //TODO actual world generation
  blocks
}
