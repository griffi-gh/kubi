use glam::{IVec3, ivec3};
use bracket_noise::prelude::*;
use super::{
  chunk::{BlockData, CHUNK_SIZE},
  block::Block
};

pub fn generate_world(chunk_position: IVec3, seed: u64) -> BlockData {
  let offset = chunk_position * CHUNK_SIZE as i32;
  
  let mut cave_noise = FastNoise::seeded(seed);
  cave_noise.set_fractal_type(FractalType::FBM);
  cave_noise.set_frequency(0.1);

  let mut dirt_noise = FastNoise::seeded(seed.rotate_left(1));
  dirt_noise.set_fractal_type(FractalType::FBM);
  dirt_noise.set_frequency(0.1);

  let mut blocks = Box::new([[[Block::Air; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]);

  if chunk_position.y >= 0 {
    if chunk_position.y == 0 {
      for x in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
          blocks[x][0][z] = Block::Dirt;
          blocks[x][1][z] = Block::Grass;
        }
      }
    }
  } else {
    for x in 0..CHUNK_SIZE {
      for y in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
          let position = ivec3(x as i32, y as i32, z as i32) + offset;
          let v_cave_noise = cave_noise.get_noise3d(position.x as f32, position.y as f32, position.z as f32) * (-position.y as f32 - 10.0).clamp(0., 1.);
          let v_dirt_noise = dirt_noise.get_noise3d(position.x as f32, position.y as f32, position.z as f32) * (-position.y as f32).clamp(0., 1.);
          if v_cave_noise > 0.5 {
            blocks[x][y][z] = Block::Stone;
          } else if v_dirt_noise > 0.5 {
            blocks[x][y][z] = Block::Dirt;
          }
        }
      }
    }
  }
  
  
  blocks
}
