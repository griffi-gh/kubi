use shipyard::{Unique, UniqueView, UniqueViewMut, Workload, IntoWorkload, AllStoragesView, View, Get, NonSendSync};
use glam::IVec3;
use hashbrown::HashMap;
use kubi_shared::networking::{
  messages::{ClientToServerMessage, ServerToClientMessage, C_CHUNK_SUB_REQUEST}, 
  channels::CHANNEL_WORLD, 
  client::Client,
};
use uflow::{
  server::Event as ServerEvent,
  SendMode
};
use crate::{
  server::{UdpServer, ServerEvents, IsMessageOfType}, 
  config::ConfigTable,
  client::{ClientAddress, ClientIdMap, ClientAddressMap}, 
};

pub mod chunk;
pub mod tasks;

use chunk::Chunk;

use self::{tasks::{ChunkTaskManager, ChunkTask, ChunkTaskResponse, init_chunk_task_manager}, chunk::ChunkState};

#[derive(Unique, Default)]
pub struct ChunkManager {
  pub chunks: HashMap<IVec3, Chunk>
}
impl ChunkManager {
  pub fn new() -> Self {
    Self::default()
  }
}

fn process_chunk_requests(
  mut server: NonSendSync<UniqueViewMut<UdpServer>>,
  events: UniqueView<ServerEvents>,
  mut chunk_manager: UniqueViewMut<ChunkManager>,
  task_manager: UniqueView<ChunkTaskManager>,
  config: UniqueView<ConfigTable>,
  addr_map: UniqueView<ClientAddressMap>,
  clients: View<Client>
) {
  for event in &events.0 {
    let ServerEvent::Receive(client_addr, data) = event else{
      continue
    };
    if !event.is_message_of_type::<C_CHUNK_SUB_REQUEST>() {
      continue
    }
    let Some(client) = server.0.client(client_addr) else {
      log::error!("Client doesn't exist");
      continue
    };
    let Some(&entity_id) = addr_map.0.get(client_addr) else {
      log::error!("Client not authenticated");
      continue
    };
    let Ok(&Client(client_id)) = (&clients).get(entity_id) else {
      log::error!("Entity ID is invalid");
      continue
    };
    let Ok(parsed_message) = postcard::from_bytes(data) else {
      log::error!("Malformed message");
      continue
    };
    let ClientToServerMessage::ChunkSubRequest { chunk: chunk_position } = parsed_message else {
      unreachable!()
    };

    if let Some(chunk) = chunk_manager.chunks.get_mut(&chunk_position) {
      chunk.subscriptions.insert(client_id);
      //TODO Start task here if status is "Nothing"
      if let Some(blocks) = &chunk.blocks {
        client.borrow_mut().send(
          postcard::to_allocvec(&ServerToClientMessage::ChunkResponse {
            chunk: chunk_position,
            data: blocks.clone(),
            queued: Vec::with_capacity(0)
          }).unwrap().into_boxed_slice(),
          CHANNEL_WORLD,
          SendMode::Reliable,
        );
      }
    } else {
      let mut chunk = Chunk::new(chunk_position);
      chunk.state = ChunkState::Loading;
      chunk.subscriptions.insert(client_id);
      chunk_manager.chunks.insert(chunk_position, chunk);
      task_manager.spawn_task(ChunkTask::LoadChunk {
        position: chunk_position,
        seed: config.world.seed,
      });
    }
  }
}

fn process_finished_tasks(
  mut server: NonSendSync<UniqueViewMut<UdpServer>>,
  task_manager: UniqueView<ChunkTaskManager>,
  mut chunk_manager: UniqueViewMut<ChunkManager>,
  id_map: UniqueView<ClientIdMap>,
  client_addr: View<ClientAddress>,
) {
  let mut limit: usize = 8;
  'outer: while let Some(res) = task_manager.receive() {
    let ChunkTaskResponse::ChunkLoaded { chunk_position, blocks, queue } = res;
    let Some(chunk) = chunk_manager.chunks.get_mut(&chunk_position) else {
      log::warn!("Chunk discarded: Doesn't exist");
      continue
    };
    if chunk.state != ChunkState::Loading {
      log::warn!("Chunk discarded: Not Loading");
      continue
    }
    chunk.state = ChunkState::Loaded;
    chunk.blocks = Some(blocks.clone());
    log::debug!("Chunk {chunk_position} loaded, {} subs", chunk.subscriptions.len());
    for &subscriber in &chunk.subscriptions {
      let Some(&entity_id) = id_map.0.get(&subscriber) else {
        log::error!("Invalid subscriber client id");
        continue 'outer;
      };
      let Ok(&ClientAddress(client_addr)) = (&client_addr).get(entity_id) else {
        log::error!("Invalid subscriber entity id");
        continue 'outer;
      };
      let Some(client) = server.0.client(&client_addr) else {
        log::error!("Client not connected");
        continue 'outer;
      };
      client.borrow_mut().send(
        postcard::to_allocvec(&ServerToClientMessage::ChunkResponse {
          chunk: chunk_position,
          data: blocks.clone(),
          queued: queue.clone()
        }).unwrap().into_boxed_slice(),
        CHANNEL_WORLD,
        SendMode::Reliable,
      );
    }
  }
}

fn init_chunk_manager(
  storages: AllStoragesView
) {
  storages.add_unique(ChunkManager::new());
}

pub fn init_world() -> Workload {
  (
    init_chunk_manager,
    init_chunk_task_manager,
  ).into_workload()
}

pub fn update_world() -> Workload {
  (
    process_chunk_requests,
    process_finished_tasks,
  ).into_workload()
}
