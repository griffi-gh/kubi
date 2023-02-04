use std::collections::VecDeque;
use glam::IVec3;
use kubi_shared::blocks::Block;
use shipyard::{UniqueViewMut, Unique};

use super::ChunkStorage;

#[derive(Clone, Copy, Debug)]
pub struct BlockUpdateEvent {
  pub position: IVec3,
  pub value: Block
}

#[derive(Unique, Default, Clone)]
pub struct BlockUpdateQueue {
  queue: VecDeque<BlockUpdateEvent>
}
impl BlockUpdateQueue {
  pub fn new() -> Self {
    Self::default()
  }
  pub fn push(&mut self, event: BlockUpdateEvent) {
    self.queue.push_back(event)
  }
  pub fn pop(&mut self) -> Option<BlockUpdateEvent> {
    self.queue.pop_front()
  }
  pub fn clear(&mut self) {
    self.queue.clear();
  }
}

pub fn apply_events(
  mut queue: UniqueViewMut<BlockUpdateQueue>,
  mut world: UniqueViewMut<ChunkStorage>
) {
  while let Some(event) = queue.pop() {
    if let Some(block) = world.get_block_mut(event.position) {
      let (chunk_pos, _) = ChunkStorage::to_chunk_coords(event.position);
      let chunk = world.chunks.get_mut(&chunk_pos).expect("This error should never happen, if it does then something is super fucked up and the whole project needs to be burnt down.");
      chunk.dirty = true;
    }
    
  }
}
