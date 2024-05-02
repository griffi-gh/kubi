use glam::ivec3;
use crate::{block::Block, chunk::CHUNK_SIZE};
use super::{WorldGenerator, WorldGenStep};

pub const WATER_LEVEL: i32 = 0;

pub struct WaterStep;

impl WorldGenStep for WaterStep {
  fn initialize(_: &WorldGenerator) -> Self { Self }
  fn generate(&mut self, gen: &mut WorldGenerator) {
    for x in 0..CHUNK_SIZE as i32 {
      for z in 0..CHUNK_SIZE as i32 {
        for y in 0..gen.local_height(WATER_LEVEL) {
          gen.place_if_empty(ivec3(x, y, z), Block::Water);
        }
      }
    }
  }
}
