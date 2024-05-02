use fastnoise_lite::{FastNoiseLite, FractalType};
use glam::ivec3;
use crate::{block::Block, chunk::CHUNK_SIZE};
use super::{SeedThingy, WorldGenStep, WorldGenerator};

pub struct TerrainStep {
  noise: FastNoiseLite,
}

impl WorldGenStep for TerrainStep {
  fn initialize(generator: &WorldGenerator) -> Self {
    let mut seeder = SeedThingy::new(generator.seed);
    let mut noise = FastNoiseLite::with_seed(seeder.next_seed());
    noise.set_fractal_type(Some(FractalType::FBm));
    noise.set_fractal_octaves(Some(4));
    noise.set_frequency(Some(0.003));
    Self { noise }
  }

  fn generate(&mut self, gen: &mut WorldGenerator) {
    for x in 0..CHUNK_SIZE as i32 {
      for z in 0..CHUNK_SIZE as i32 {
        let global_xz = gen.global_position(ivec3(x, 0, z));
        let height = (self.noise.get_noise_2d(global_xz.x as f64, global_xz.z as f64) * 32.0) as i32;
        for y in 0..gen.local_height(height) {
          gen.place(ivec3(x, y, z), Block::Stone);
        }
      }
    }
  }
}
