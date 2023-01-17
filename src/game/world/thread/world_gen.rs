use glam::IVec2;
use simdnoise::NoiseBuilder;
use crate::game::{
  world::chunk::{ChunkData, CHUNK_SIZE, CHUNK_HEIGHT}, 
  blocks::Block
};

const TERRAIN_HEIGHT_MIN: f32 = 64.;
const TERRAIN_HEIGHT_MAX: f32 = 80.;

pub fn generate_chunk(position: IVec2, seed: u32) -> ChunkData {
  let world_xz = position.as_vec2() * CHUNK_SIZE as f32;
  let mut chunk = Box::new([[[Block::Air; CHUNK_SIZE]; CHUNK_HEIGHT]; CHUNK_SIZE]);

  //generate noises
  let height_noise = NoiseBuilder::fbm_2d_offset(world_xz.x, CHUNK_SIZE, world_xz.y, CHUNK_SIZE)
    .with_freq(0.01)
    .with_octaves(4)
    .with_seed(seed as i32)
    .generate_scaled(TERRAIN_HEIGHT_MIN, TERRAIN_HEIGHT_MAX);

  //put everything together
  for x in 0..CHUNK_SIZE {
    for z in 0..CHUNK_SIZE {
      let heightmap = height_noise[x + z * CHUNK_SIZE] as usize;
      for y in 0..heightmap {
        chunk[x][y][z] = Block::Dirt;
      }
      chunk[x][heightmap][z] = Block::Grass;
    }
  }

  //return generated world
  chunk
}
