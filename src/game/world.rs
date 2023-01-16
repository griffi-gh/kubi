use glam::{Vec2, IVec2};
use glium::Display;
use std::collections::HashMap;
use crate::game::options::GameOptions;

mod chunk;
mod thread;

use chunk::{Chunk, ChunkState, CHUNK_SIZE};
use thread::WorldThreading;

const POSITIVE_X_NEIGHBOR: usize = 0;
const NEGATIVE_X_NEIGHBOR: usize = 1;
const POSITIVE_Z_NEIGHBOR: usize = 2;
const NEGATIVE_Z_NEIGHBOR: usize = 3;

pub struct World {
  pub chunks: HashMap<IVec2, Chunk>,
  pub thread: WorldThreading,
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
      chunks: HashMap::new(),
      thread: WorldThreading::new(),
    }
  }
  pub fn update_loaded_chunks(&mut self, around_position: Vec2, options: &GameOptions, display: &Display) {
    let render_dist = options.render_distance as i32 + 1;
    let inside_chunk = (around_position / CHUNK_SIZE as f32).as_ivec2();
    //Mark all chunks for unload
    for (_, chunk) in &mut self.chunks {
      chunk.desired = ChunkState::Unload;
    }
    //Load new/update chunks in range
    for x in -render_dist..=render_dist {
      for z in -render_dist..=render_dist {
        let offset = IVec2::new(x, z);
        let position = inside_chunk + offset;
        if !self.chunks.contains_key(&position) {
          self.chunks.insert(position, Chunk::new(position));
        }
        let chunk = self.chunks.get_mut(&position).unwrap();
        if x == 0 || z == 0 || x == render_dist || z == render_dist {
          chunk.desired = ChunkState::Loaded;
        } else {
          chunk.desired = ChunkState::Rendered;
        }
      }
    }
    //State up/downgrades are handled here!
    self.chunks.retain(|&position, chunk| {
      match chunk.desired {
        // Any => Unload downgrade
        ChunkState::Unload => {
          return false
        },
        // Any => Nothing downgrade
        ChunkState::Nothing => {
          chunk.block_data = None;
          chunk.vertex_buffer = None;
          chunk.state = ChunkState::Nothing;
        },
        // Nothing => Loading => Loaded upgrade
        ChunkState::Loaded if matches!(chunk.state, ChunkState::Nothing) => {
          self.thread.queue_load(position);
        },
        //Render => Loaded downgrade
        ChunkState::Loaded if matches!(chunk.state, ChunkState::Rendering | ChunkState::Rendered) => {
          chunk.vertex_buffer = None;
          chunk.state = ChunkState::Loaded;
        },
        _ => ()
      }
      true
    });
    //Apply changes from threads
    self.thread.apply_tasks(&mut self.chunks, display);
  }
}
