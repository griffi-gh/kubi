use flume::{Sender, Receiver};
use glam::IVec3;
use shipyard::Unique;
use rayon::{ThreadPool, ThreadPoolBuilder};
use super::{
  chunk::BlockData,
  render::ChunkVertex, 
  mesh::{generate_mesh, data::MeshGenData},
  worldgen::generate_world,
};

pub enum ChunkTask {
  LoadChunk {
    seed: u64,
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

#[derive(Unique)]
pub struct ChunkTaskManager {
  channel: (Sender<ChunkTaskResponse>, Receiver<ChunkTaskResponse>),
  pool: ThreadPool,
}
impl ChunkTaskManager {
  pub fn new() -> Self {
    Self {
      channel: flume::unbounded::<ChunkTaskResponse>(), //maybe put a bound or even bound(0)?
      pool: ThreadPoolBuilder::new().num_threads(4).build().unwrap()
    }
  }
  pub fn spawn_task(&self, task: ChunkTask) {
    let sender = self.channel.0.clone();
    self.pool.spawn(move || {
      let _ = sender.send(match task {
        ChunkTask::GenerateMesh { position, data } => {
          let (vertices, indexes) = generate_mesh(data);
          ChunkTaskResponse::GeneratedMesh { position, vertices, indexes }
        },
        ChunkTask::LoadChunk { position, seed } => {
          let chunk_data = generate_world(position, seed);
          ChunkTaskResponse::LoadedChunk { position, chunk_data }
        }
      });
    });
  }
  pub fn receive(&self) -> Option<ChunkTaskResponse> {
    self.channel.1.try_recv().ok()
  }
}
