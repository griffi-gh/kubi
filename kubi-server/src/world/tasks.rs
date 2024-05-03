use shipyard::{Unique, AllStoragesView};
use flume::{unbounded, Sender, Receiver};
use glam::IVec3;
use rayon::{ThreadPool, ThreadPoolBuilder};
use anyhow::Result;
use kubi_shared::{
  chunk::BlockData, 
  worldgen::generate_world,
  queue::QueuedBlock,
};

pub enum ChunkTask {
  LoadChunk {
    position: IVec3,
    seed: u64,
  }
}

pub enum ChunkTaskResponse {
  ChunkLoaded {
    chunk_position: IVec3,
    blocks: BlockData,
    queue: Vec<QueuedBlock>
  }
}

#[derive(Unique)]
pub struct ChunkTaskManager {
  channel: (Sender<ChunkTaskResponse>, Receiver<ChunkTaskResponse>),
  pool: ThreadPool,
}
impl ChunkTaskManager {
  pub fn new() -> Result<Self> {
    Ok(Self {
      channel: unbounded(),
      pool: ThreadPoolBuilder::new().build()?
    })
  }
  pub fn spawn_task(&self, task: ChunkTask) {
    let sender = self.channel.0.clone();
    self.pool.spawn(move || {
      sender.send(match task {
        ChunkTask::LoadChunk { position: chunk_position, seed } => {
          //unwrap is fine because abort is not possible
          let (blocks, queue) = generate_world(chunk_position, seed, None).unwrap();
          ChunkTaskResponse::ChunkLoaded { chunk_position, blocks, queue }
        }
      }).unwrap()
    })
  }
  pub fn receive(&self) -> Option<ChunkTaskResponse> {
    self.channel.1.try_recv().ok()
  }
}

pub fn init_chunk_task_manager(
  storages: AllStoragesView
) {
  storages.add_unique(ChunkTaskManager::new().expect("ChunkTaskManager Init failed"));
}
