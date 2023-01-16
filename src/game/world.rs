use glam::{Vec2, IVec2};
use std::collections::HashMap;
use crate::game::options::GameOptions;

mod chunk;
mod thread;

use chunk::{Chunk, CHUNK_SIZE};

const POSITIVE_X_NEIGHBOR: usize = 0;
const NEGATIVE_X_NEIGHBOR: usize = 1;
const POSITIVE_Z_NEIGHBOR: usize = 2;
const NEGATIVE_Z_NEIGHBOR: usize = 3;

pub struct World {
  pub chunks: HashMap<IVec2, Chunk>
}
impl World {
  pub fn chunk_neighbors(&self, position: IVec2) -> [Option<&Chunk>; 4] {
    [
      self.chunks.get(&(position + IVec2::new(1, 0))),
      self.chunks.get(&(position - IVec2::new(1, 0))),
      self.chunks.get(&(position + IVec2::new(0, 1))),
      self.chunks.get(&(position - IVec2::new(0, 1))),
    ]
  }
  pub fn new() -> Self {
    Self {
      chunks: HashMap::new()
    }
  }
  pub fn update_loaded_chunks(&mut self, around_position: Vec2, game_opt: &GameOptions) {
    let render_dist = game_opt.render_distance as i32;
    let inside_chunk = (around_position / CHUNK_SIZE as f32).as_ivec2();
    
    todo!()
  }
}
