use bracket_noise::prelude::{FastNoise, FractalType};
use glam::{ivec3, IVec3};
use crate::{block::Block, chunk::CHUNK_SIZE};
use super::{WorldGenStep, WorldGenerator};

pub struct CaveStep {
  a: FastNoise,
  b: FastNoise,
}

impl WorldGenStep for CaveStep {
  fn initialize(gen: &WorldGenerator) -> Self {
    let mut a = FastNoise::seeded(gen.seed);
    a.set_fractal_type(FractalType::FBM);
    a.set_frequency(0.015);

    let mut b = FastNoise::seeded(gen.seed.rotate_left(1) + 1);
    b.set_fractal_type(FractalType::FBM);
    b.set_frequency(0.015);

    Self { a, b }
  }
  fn generate(&mut self, gen: &mut WorldGenerator) {
    for x in 0..CHUNK_SIZE as i32 {
      for z in 0..CHUNK_SIZE as i32 {
        for y in 0..CHUNK_SIZE as i32 {
          let pos: IVec3 = ivec3(x, y, z);
          if gen.query(pos) != Block::Stone { continue }

          let gpos = gen.global_position(pos);
          let noise_a = self.a.get_noise3d(gpos.x as f32, gpos.y as f32, gpos.z as f32);
          let noise_b = self.b.get_noise3d(gpos.x as f32, gpos.y as f32, gpos.z as f32);
          let noise_min = noise_a.min(noise_b);

          if noise_min > 0.5 { return }

          //gen.place(ivec3(x, y, z), Block::Air);
        }
      }
    }
  }
}
