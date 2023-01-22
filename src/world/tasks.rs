use flume::{Sender, Receiver};
use glam::IVec3;
use super::{
  chunk::BlockData,
  render::ChunkVertex
};

pub enum ChunkTask {
  LoadChunk {
    position: IVec3
  },
  GenerateMesh {
    position: IVec3,
    
  }
}

pub enum ChunkTaskResponse {
  LoadedChunk {
    position: IVec3,
    chunk_data: BlockData,
  },
  GeneratedMesh {
    position: IVec3,
    vertices: Vec<ChunkVertex>,
    indexes: Vec<u32>
  },
}

pub struct ChunkTaskManager {
  channel: (Sender<ChunkTaskResponse>, Receiver<ChunkTaskResponse>),
}
impl ChunkTaskManager {
  pub fn new() -> Self {
    Self {
      channel: flume::bounded::<ChunkTaskResponse>(0),
    }
  }
  pub fn spawn_task() {
    
  }
}
