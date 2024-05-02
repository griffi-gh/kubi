use std::sync::atomic::AtomicBool;

use glam::{ivec3, IVec3};
use crate::{
  block::Block,
  chunk::{BlockData, CHUNK_SIZE},
  queue::QueuedBlock,
};

mod _01_terrain;

trait WorldGenStep {
  fn initialize(generator: &WorldGenerator) -> Self;
  fn generate(&mut self, generator: &mut WorldGenerator);
}

macro_rules! run_steps {
  ($gen: expr, $abort: expr, [$($step:ty),* $(,)?]) => {
    {
      let _abort: AtomicBool = $abort.unwrap_or(AtomicBool::new(false));
      $({
        let _ensure_ref: &mut WorldGenerator = $gen;
        struct _Ensure0<T: WorldGenStep>(T);
        type _Ensure1 = _Ensure0<$step>;
        let mut step: _Ensure1 = _Ensure0(<$step>::initialize(&*_ensure_ref));
        step.0.generate(_ensure_ref);
      })*
    }
  };
}

pub struct WorldGenerator {
  seed: u64,
  chunk_position: IVec3,
  blocks: BlockData,
  queue: Vec<QueuedBlock>,
}

impl WorldGenerator {
  fn offset(&self) -> IVec3 {
    self.chunk_position * CHUNK_SIZE as i32
  }

  fn place_or_queue(&mut self, position: IVec3, block: Block) {
    let offset = self.offset();
    if position.to_array().iter().any(|&x| !(0..CHUNK_SIZE).contains(&(x as usize))) {
      let event_pos = offset + position;
      self.queue.retain(|block: &QueuedBlock| {
        block.position != event_pos
      });
      self.queue.push(QueuedBlock {
        position: event_pos,
        block_type: block,
        soft: true
      });
    } else {
      self.blocks[position.x as usize][position.y as usize][position.z as usize] = block;
    }
  }

  pub fn new(chunk_position: IVec3, seed: u64) -> Self {
    Self {
      seed,
      chunk_position,
      blocks: Box::new([[[Block::Air; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]),
      queue: Vec::with_capacity(0),
    }
  }

  pub fn generate(mut self, abort: Option<AtomicBool>) -> (BlockData, Vec<QueuedBlock>) {
    run_steps!(&mut self, abort, [
      _01_terrain::TerrainStep
    ]);
    (self.blocks, self.queue)
  }
}

pub fn generate_world(chunk_position: IVec3, seed: u64) -> (BlockData, Vec<QueuedBlock>) {
  WorldGenerator::new(chunk_position, seed).generate(None)
}
