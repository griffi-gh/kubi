use glam::{IVec3, ivec3};
use bracket_noise::prelude::*;
use super::{
  chunk::{BlockData, CHUNK_SIZE},
  block::Block
};

pub fn generate_world(chunk_position: IVec3, seed: u64) -> BlockData {
  let offset = chunk_position * CHUNK_SIZE as i32;
  let mut noise = FastNoise::seeded(seed);
  noise.set_fractal_type(FractalType::FBM);
  noise.set_frequency(0.1);

  let mut blocks = Box::new([[[Block::Air; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]);

  // blocks[0][0][0] = Block::Stone;
  // blocks[0][CHUNK_SIZE - 1][0] = Block::Stone;
  // blocks[CHUNK_SIZE - 1][0][0] = Block::Stone;
  // blocks[CHUNK_SIZE - 1][CHUNK_SIZE - 1][0] = Block::Stone;
  // blocks[0][0][CHUNK_SIZE - 1] = Block::Stone;
  // blocks[0][CHUNK_SIZE - 1][CHUNK_SIZE - 1] = Block::Stone;
  // blocks[CHUNK_SIZE - 1][0][CHUNK_SIZE - 1] = Block::Stone;
  // blocks[CHUNK_SIZE - 1][CHUNK_SIZE - 1][CHUNK_SIZE - 1] = Block::Stone;

  for x in 0..CHUNK_SIZE {
    for y in 0..CHUNK_SIZE {
      for z in 0..CHUNK_SIZE {
        let position = ivec3(x as i32, y as i32, z as i32) + offset;
        let noise = noise.get_noise3d(position.x as f32, position.y as f32, position.z as f32);
        if (0.7..0.8).contains(&noise) {
          blocks[x][y][z] = Block::Stone;
        }
      }
    }
  }
  
  blocks
}
