use glam::{Vec2, DVec2, IVec2};
use noise::{NoiseFn, Perlin, Simplex, Fbm, Seedable};
use crate::game::{
  world::chunk::{ChunkData, CHUNK_SIZE, CHUNK_HEIGHT}, 
  blocks::Block
};

const HEIGHTMAP_SCALE: f64 = 0.004;
const MOUNTAINESS_SCALE: f64 = 0.0001;
const MNT_RAMP_1: f64 = 0.5;
const MNT_RAMP_2: f64 = 0.6;
const MTN_VAL_SCALE: f64 = 1.233;
const TERRAIN_HEIGHT_MIN: f64 = 60.;
const TERRAIN_HEIGHT_MAX: f64 = 80.;

pub fn generate_chunk(position: IVec2, seed: u32) -> ChunkData {
  let world_xz = position.as_vec2() * CHUNK_SIZE as f32;
  let mut chunk = Box::new([[[Block::Air; CHUNK_SIZE]; CHUNK_HEIGHT]; CHUNK_SIZE]);

  //generate noises
  let mut terrain_base_fbm: Fbm<Perlin> = Fbm::new(seed);
  terrain_base_fbm.octaves = 6;
  
  let mut mountainess_base_fbm: Fbm<Perlin> = Fbm::new(seed);
  mountainess_base_fbm.octaves = 4;

  //put everything together
  for x in 0..CHUNK_SIZE {
    for z in 0..CHUNK_SIZE {
      let point = world_xz.as_dvec2() + DVec2::from_array([x as f64, z as f64]);

      let heightmap = (terrain_base_fbm.get((point * HEIGHTMAP_SCALE).to_array()) + 1.) / 2.;
      let mountainess = MTN_VAL_SCALE * ((mountainess_base_fbm.get((point * MOUNTAINESS_SCALE).to_array()) + 1.) / 2.);

      //generate basic terrain
      let terain_height = 
        (
          TERRAIN_HEIGHT_MIN + 
          (heightmap * TERRAIN_HEIGHT_MAX * (0.1 + 1.5 * if mountainess < MNT_RAMP_1 {
            0.
          } else {
            if mountainess > MNT_RAMP_2 {
              1.
            } else {
              (mountainess - MNT_RAMP_1) / (MNT_RAMP_2 - MNT_RAMP_1) * 1.
            }
          }))
        ).floor() as usize;
      for y in 0..terain_height {
        chunk[x][y][z] = Block::Dirt;
      }
      chunk[x][terain_height][z] = Block::Grass;
    }
  }

  //return generated world
  chunk
}
