use glam::IVec2;
use glium::{Display, VertexBuffer, IndexBuffer, index::PrimitiveType};
use std::{mem, thread::{self, JoinHandle}};
use hashbrown::HashMap;
use super::chunk::{Chunk, ChunkData, ChunkState};
use crate::game::shaders::chunk::Vertex as ChunkVertex;

mod world_gen;
mod mesh_gen;

#[derive(Default)]
pub struct WorldThreading {
  //drain_filter is not stable yet so
  //Options are needed here to take ownership, 
  //None values should never appear here!
  pub load_tasks: HashMap<IVec2, Option<JoinHandle<ChunkData>>>,
  pub mesh_tasks: HashMap<IVec2, Option<JoinHandle<(Vec<ChunkVertex>, Vec<u16>)>>>,
}
impl WorldThreading {
  pub fn new() -> Self {
    Self::default()
  }
  pub fn is_done(&self) -> bool {
    self.load_tasks.is_empty() && 
    self.mesh_tasks.is_empty()
  }
  pub fn task_amount(&self) -> usize {
    self.load_tasks.len() + self.mesh_tasks.len()
  }
  pub fn queue_load(&mut self, position: IVec2) {
    let handle = thread::spawn(|| {
      world_gen::generate_chunk()
    });
    if self.load_tasks.insert(position, Some(handle)).is_some() {
      log::warn!("load: discarded {}, reason: new task started", position);
    }
  }
  pub fn queue_mesh(&mut self, position: IVec2, chunk: ChunkData, neighbor_data: [ChunkData; 4]) {
    let handle = thread::spawn(move || {
      mesh_gen::generate_mesh(position, chunk, neighbor_data)
    });
    if self.mesh_tasks.insert(position, Some(handle)).is_some() {
      log::warn!("mesh: discarded {}, reason: new task started", position);
    }
  }
  pub fn apply_tasks(&mut self, chunks: &mut HashMap<IVec2, Chunk>, display: &Display) {
    //LOAD TASKS
    self.load_tasks.retain(|position, handle| {
      if !chunks.contains_key(position) {
        log::warn!("load: discarded {}, reason: chunk no longer exists", position);
        return false
      }
      if !matches!(chunks.get(position).unwrap().desired, ChunkState::Loaded | ChunkState::Rendered) {
        log::warn!("load: discarded {}, reason: state undesired", position);
        return false
      }
      if !handle.as_ref().expect("Something went terribly wrong").is_finished() {
        //task not finished yet, keep it and wait
        return true
      }
      log::info!("load: done {}", position);
      let handle = mem::take(handle).unwrap();
      let data = handle.join().unwrap();
      let chunk = chunks.get_mut(position).unwrap();
      chunk.block_data = Some(data);
      chunk.state = ChunkState::Loaded;
      false
    });
    //MESH TASKS
    self.mesh_tasks.retain(|position, handle| {
      if !chunks.contains_key(position) {
        log::warn!("mesh: discarded {}, reason: chunk no longer exists", position);
        return false
      }
      if !matches!(chunks.get(position).unwrap().desired, ChunkState::Rendered) {
        log::warn!("mesh: discarded {}, reason: state undesired", position);
        return false
      }
      if !handle.as_ref().expect("Something went terribly wrong").is_finished() {
        //task not finished yet, keep it and wait
        return true
      }
      log::info!("mesh: done {}", position);
      let handle = mem::take(handle).unwrap();
      let (shape, index) = handle.join().unwrap();
      let chunk = chunks.get_mut(position).unwrap();
      chunk.mesh = Some(( 
        true,
        VertexBuffer::immutable(display, &shape).expect("Failed to build VertexBuffer"),
        IndexBuffer::immutable(display, PrimitiveType::TrianglesList, &index).expect("Failed to build IndexBuffer")
      ));
      chunk.state = ChunkState::Rendered;
      false
    });

  }
}
