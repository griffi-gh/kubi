use flume::{Sender, Receiver};
use glam::IVec3;
use kubi_shared::{
  networking::messages::{ClientToServerMessage, ServerToClientMessage}, 
  worldgen::QueuedBlock
};
use shipyard::{Unique, UniqueView, View, IntoIter};
use rayon::{ThreadPool, ThreadPoolBuilder};
use super::{
  chunk::BlockData,
  mesh::{generate_mesh, data::MeshGenData},
  worldgen::generate_world,
};
use crate::{
  rendering::world::ChunkVertex, 
  networking::{UdpClient, NetworkEvent}
};
use kubi_udp::client::ClientEvent;

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
          let (vertices, indexes) = generate_mesh(data);
          ChunkTaskResponse::GeneratedMesh { position, vertices, indexes }
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

pub fn spawn_task_or_get_from_network_if_possible(client: Option<&mut UdpClient>, manager: &mut ChunkTaskManager, task: ChunkTask) {
  match &task {
    ChunkTask::LoadChunk { seed, position } => {
      match client {
        Some(client) => {
          client.0.send_message(ClientToServerMessage::ChunkRequest { chunk: position.to_array() }).unwrap();
        },
        None => {
          manager.spawn_task(task)
        }
      }
    },
    _ => {
      manager.spawn_task(task)
    }
  }
}

//TODO get rid of this, this is awfulll
pub fn inject_network_responses_into_manager_queue(
  manager: UniqueView<ChunkTaskManager>,
  events: View<NetworkEvent>
) {
  for event in events.iter() {
    if let ClientEvent::MessageReceived(ServerToClientMessage::ChunkResponse { chunk, data }) = &event.0 {
      let position = IVec3::from_array(*chunk);
      manager.add_sussy_response(ChunkTaskResponse::LoadedChunk {
        position, 
        chunk_data: data.clone(),
        queued: Vec::with_capacity(0)
      });
    }
  }
}
