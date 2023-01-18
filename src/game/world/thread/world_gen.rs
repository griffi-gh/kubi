use glam::{Vec2, DVec2, IVec2};
use noise::{NoiseFn, Perlin, Simplex, Fbm, Seedable};
use crate::game::{
  world::chunk::{ChunkData, CHUNK_SIZE, CHUNK_HEIGHT}, 
  blocks::Block
};

const HEIGHTMAP_SCALE: f64 = 0.004;
const TERRAIN_HEIGHT_MIN: f64 = 64.;
const TERRAIN_HEIGHT_MAX: f64 = 80.;

pub fn generate_chunk(position: IVec2, seed: u32) -> ChunkData {
  let world_xz = position.as_vec2() * CHUNK_SIZE as f32;
  let mut chunk = Box::new([[[Block::Air; CHUNK_SIZE]; CHUNK_HEIGHT]; CHUNK_SIZE]);

  //generate noises
  let mut terrain_base_fbm: Fbm<Perlin> = Fbm::new(seed);
  terrain_base_fbm.octaves = 6;
  
  //put everything together
  for x in 0..CHUNK_SIZE {
    for z in 0..CHUNK_SIZE {
      let point = world_xz.as_dvec2() + DVec2::from_array([x as f64, z as f64]);

      let heightmap = (terrain_base_fbm.get((point * HEIGHTMAP_SCALE).to_array()) + 1.) / 2.;
      //generate basic terrain
      let terain_height = (TERRAIN_HEIGHT_MIN + (heightmap * TERRAIN_HEIGHT_MAX)).floor() as usize;
      for y in 0..terain_height {
        chunk[x][y][z] = Block::Dirt;
      }
      chunk[x][terain_height][z] = Block::Grass;
    }
  }

  //return generated world
  chunk
}
