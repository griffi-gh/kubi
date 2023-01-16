use glam::{Vec2, IVec2};
use std::collections::HashMap;
use crate::game::options::GameOptions;

mod chunk;
mod thread;

use chunk::{Chunk, CHUNK_SIZE};

pub struct World {
  pub chunks: HashMap<IVec2, Chunk>
}
impl World {
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
