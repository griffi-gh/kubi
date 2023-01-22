use flume::{Sender, Receiver};
use glam::IVec3;
use super::{
  chunk::BlockData,
  render::ChunkVertex, 
  mesh::data::MeshGenData,
  worldgen::generate_world,
};

pub enum ChunkTask {
  LoadChunk {
    seed: u32,
    position: IVec3
  },
  GenerateMesh {
    position: IVec3,
    data: MeshGenData
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
  pub fn spawn_task(&self, task: ChunkTask) {
    let sender = self.channel.0.clone();
    rayon::spawn(move || {
      sender.send(match task {
        ChunkTask::GenerateMesh { position, data } => {
          todo!()
        },
        ChunkTask::LoadChunk { position, seed } => {
          let chunk_data = generate_world(position, seed);
          ChunkTaskResponse::LoadedChunk { position, chunk_data }
        }
      });
    });
  }
}
