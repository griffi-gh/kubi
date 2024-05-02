use bracket_noise::prelude::{FastNoise, FractalType};
use glam::ivec3;
use crate::{block::Block, chunk::CHUNK_SIZE};
use super::{WorldGenerator, WorldGenStep};

pub struct TerrainStep {
  noise: FastNoise,
}

impl WorldGenStep for TerrainStep {
  fn initialize(generator: &WorldGenerator) -> Self {
    let mut noise = FastNoise::seeded(generator.seed);
    noise.set_fractal_type(FractalType::RigidMulti);
    noise.set_fractal_octaves(5);
    noise.set_frequency(0.003);
    Self { noise }
  }

  fn generate(&mut self, gen: &mut WorldGenerator) {
    for x in 0..CHUNK_SIZE as i32 {
      for z in 0..CHUNK_SIZE as i32 {
        let global_xz = gen.global_position(ivec3(x, 0, z));
        let height = (self.noise.get_noise(global_xz.x as f32, global_xz.z as f32) * 32.0) as i32;
        for y in 0..gen.local_height(height) {
          gen.place(ivec3(x, y, z), Block::Stone);
        }
      }
    }
  }
}
