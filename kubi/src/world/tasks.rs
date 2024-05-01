use flume::{Sender, Receiver};
use glam::IVec3;
use kubi_shared::queue::QueuedBlock;
use shipyard::Unique;
use rayon::{ThreadPool, ThreadPoolBuilder};
use super::{
  chunk::BlockData,
  mesh::{generate_mesh, data::MeshGenData},
  worldgen::generate_world,
};
use crate::rendering::world::ChunkVertex;

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
    queued: Vec<QueuedBlock>
  },
  GeneratedMesh {
    position: IVec3,
    vertices: Vec<ChunkVertex>,
    indices: Vec<u32>,
    trans_vertices: Vec<ChunkVertex>,
    trans_indices: Vec<u32>,
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
  pub fn add_sussy_response(&self, response: ChunkTaskResponse) {
    // this WILL get stuck if the channel is bounded
    // don't make the channel bounded ever
    self.channel.0.send(response).unwrap()
  }
  pub fn spawn_task(&self, task: ChunkTask) {
    let sender = self.channel.0.clone();
    self.pool.spawn(move || {
      let _ = sender.send(match task {
        ChunkTask::GenerateMesh { position, data } => {
          let (
            (vertices, indices),
            (trans_vertices, trans_indices),
          ) = generate_mesh(data);
          ChunkTaskResponse::GeneratedMesh {
            position,
            vertices, indices,
            trans_vertices, trans_indices,
          }
        },
        ChunkTask::LoadChunk { position, seed } => {
          let (chunk_data, queued) = generate_world(position, seed);
          ChunkTaskResponse::LoadedChunk { position, chunk_data, queued }
        }
      });
    });
  }
  pub fn receive(&self) -> Option<ChunkTaskResponse> {
    self.channel.1.try_recv().ok()
  }
}
