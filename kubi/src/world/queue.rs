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
  queue: Vec<BlockUpdateEvent>
}
impl BlockUpdateQueue {
  pub fn new() -> Self {
    Self::default()
  }
  pub fn push(&mut self, event: BlockUpdateEvent) {
    self.queue.push(event)
  }
}

pub fn apply_queued_blocks(
  mut queue: UniqueViewMut<BlockUpdateQueue>,
  mut world: UniqueViewMut<ChunkStorage>
) {
  queue.queue.retain(|&event| {
    if let Some(block) = world.get_block_mut(event.position) {
      *block = event.value;
      //mark chunk as dirty
      //maybe i need to check for desired/current state here?
      let (chunk_pos, _) = ChunkStorage::to_chunk_coords(event.position);
      let chunk = world.chunks.get_mut(&chunk_pos).expect("This error should never happen, if it does then something is super fucked up and the whole project needs to be burnt down.");
      chunk.mesh_dirty = true;
      return false
    }
    true
  });
}
