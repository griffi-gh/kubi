use glam::IVec2;
use std::{
  thread::{self, JoinHandle}, 
  collections::HashMap,
  mem
};
use super::chunk::{ChunkData, Chunk, DesiredState};

mod world_gen;
mod mesh_gen;

struct WorldThreading {
  //drain_filter is not stable yet so
  //Options are needed here to take ownership, 
  //None values should never appear here!
  pub load_tasks: HashMap<IVec2, Option<JoinHandle<ChunkData>>>,
  pub mesh_tasks: HashMap<IVec2, Option<JoinHandle<ChunkData>>>,
}
impl WorldThreading {
  pub fn is_done(&self) -> bool {
    self.load_tasks.is_empty() && 
    self.mesh_tasks.is_empty()
  }
  pub fn queue_load(&mut self, position: IVec2) {
    let handle = thread::spawn(|| {
      world_gen::generate_chunk()
    });
    if self.load_tasks.insert(position, Some(handle)).is_some() {
      log::warn!("load: discarded {}, reason: new task started", position);
    }
  }
  pub fn apply_tasks(&mut self, chunks: &mut HashMap<IVec2, Chunk>) {
    self.load_tasks.retain(|position, handle| {
      if !chunks.contains_key(position) {
        log::warn!("load: discarded {}, reason: chunk no longer exists", position);
        return false
      }
      if !matches!(chunks.get(position).unwrap().desired, DesiredState::Loaded | DesiredState::Rendered) {
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
      chunks.get_mut(position).unwrap().block_data = Some(data);
      false
    });
  }
}
