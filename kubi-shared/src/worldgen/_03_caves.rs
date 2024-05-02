
use fastnoise_lite::{FastNoiseLite, FractalType};
use glam::{ivec3, IVec3};
use crate::{block::Block, chunk::CHUNK_SIZE};
use super::{SeedThingy, WorldGenStep, WorldGenerator};

pub struct CaveStep {
  a: FastNoiseLite,
  b: FastNoiseLite,
}

impl WorldGenStep for CaveStep {
  fn initialize(gen: &WorldGenerator) -> Self {
    let mut seeder = SeedThingy::new(gen.seed);

    let mut a = FastNoiseLite::with_seed(seeder.next_seed());
    a.set_fractal_type(Some(FractalType::FBm));
    a.set_fractal_octaves(Some(2));

    let mut b = FastNoiseLite::with_seed(seeder.next_seed());
    b.set_fractal_type(Some(FractalType::FBm));
    b.set_fractal_octaves(Some(2));

    Self { a, b }
  }

  fn generate(&mut self, gen: &mut WorldGenerator) {
    //TODO

    // for x in 0..CHUNK_SIZE as i32 {
    //   for y in 0..CHUNK_SIZE as i32 {
    //     for z in 0..CHUNK_SIZE as i32 {
    //       let pos = ivec3(x, y, z);
    //       if gen.query(pos) != Block::Stone { continue }
    //       let pos_global = gen.global_position(pos);
    //       let noise_a = self.a.get_noise_3d(pos_global.x as f64, pos_global.y as f64, pos_global.z as f64) * 0.5 + 0.5;
    //       let noise_b = self.b.get_noise_3d(pos_global.x as f64, pos_global.y as f64, pos_global.z as f64) * 0.5 + 0.5;
    //       if noise_a.min(noise_b) > (1. - (-y as f32 / 400.).clamp(0., 1.)) {
    //         gen.place(pos, Block::Air);
    //       }
    //       //TODO
    //     }
    //   }
    // }
  }
}
