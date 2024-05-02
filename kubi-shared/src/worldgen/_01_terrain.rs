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
    noise.set_fractal_type(FractalType::FBM);
    noise.set_fractal_octaves(4);
    noise.set_frequency(0.003);
    Self { noise }
  }

  fn generate(&mut self, generator: &mut WorldGenerator) {
    for x in 0..CHUNK_SIZE as i32 {
      for z in 0..CHUNK_SIZE as i32 {
        let height = (self.noise.get_noise(x as f32, z as f32) * 8.0) as i32;
        for y in 0..height {
          generator.place_or_queue(ivec3(x, y, z), Block::Stone);
        }
      }
    }
  }
}
