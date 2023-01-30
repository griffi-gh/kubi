use glam::{IVec3, ivec3};
use glium::{VertexBuffer, IndexBuffer, index::PrimitiveType};
use shipyard::{View, UniqueView, UniqueViewMut, IntoIter, Workload, IntoWorkload, NonSendSync};
use crate::{
  player::MainPlayer,
  transform::Transform,
  settings::GameSettings,
  rendering::Renderer
};
use super::{
  ChunkStorage, ChunkMeshStorage,
  chunk::{Chunk, DesiredChunkState, CHUNK_SIZE, ChunkMesh, CurrentChunkState, ChunkData},
  tasks::{ChunkTaskManager, ChunkTaskResponse, ChunkTask},
};

//todo limit task starts insted
const MAX_CHUNK_OPS: usize = 8;

pub fn update_loaded_world_around_player() -> Workload {
  (
    update_chunks_if_player_moved,
    unload_downgrade_chunks,
    start_required_tasks,
    process_completed_tasks,
  ).into_workload()
}

pub fn update_chunks_if_player_moved(
  v_settings: UniqueView<GameSettings>,
  v_local_player: View<MainPlayer>,
  v_transform: View<Transform>,
  mut vm_world: UniqueViewMut<ChunkStorage>,
) {
  //Check if the player actually moved 
  //TODO fix this also triggers on rotation, only activate when the player crosses the chnk border
  let Some((_, transform)) = (&v_local_player, v_transform.inserted_or_modified()).iter().next() else {
    return
  };

  //Read game settings
  let load_distance = (v_settings.render_distance + 1) as i32;

  //If it did, get it's position and current chunk
  let player_position = transform.0.to_scale_rotation_translation().2;
  let player_position_ivec3 = player_position.as_ivec3();
  let player_at_chunk = ivec3(
    player_position_ivec3.x.div_euclid(CHUNK_SIZE as i32),
    player_position_ivec3.y.div_euclid(CHUNK_SIZE as i32),
    player_position_ivec3.z.div_euclid(CHUNK_SIZE as i32),
  );

  //Then, mark *ALL* chunks with ToUnload
  for (_, chunk) in &mut vm_world.chunks {
    chunk.desired_state = DesiredChunkState::ToUnload;
  }

  //Then mark chunks that are near to the player
  for x in -load_distance..=load_distance {
    for y in -load_distance..=load_distance {
      for z in -load_distance..=load_distance {
        let chunk_pos_offset = ivec3(x, y, z);
        let chunk_pos = player_at_chunk + chunk_pos_offset;
        let is_border = {
          chunk_pos_offset.x.abs() == load_distance ||
          chunk_pos_offset.y.abs() == load_distance ||
          chunk_pos_offset.z.abs() == load_distance
        };
        //If chunk doesn't exist create it
        let chunk = match vm_world.chunks.get_mut(&chunk_pos) {
          Some(chunk) => chunk,
          None => {
            let chunk = Chunk::new(chunk_pos);
            vm_world.chunks.insert_unique_unchecked(chunk_pos, chunk);
            vm_world.chunks.get_mut(&chunk_pos).unwrap()
          }
        };
        let desired = match is_border {
          true  => DesiredChunkState::Loaded,
          false => DesiredChunkState::Rendered,
        };
        chunk.desired_state = desired;
      }
    }
  }
}

fn unload_downgrade_chunks(
  mut vm_world: UniqueViewMut<ChunkStorage>,
  mut vm_meshes: NonSendSync<UniqueViewMut<ChunkMeshStorage>>
) {
  if !vm_world.is_modified() {
    return
  }
  //TODO refactor this
  vm_world.chunks.retain(|_, chunk| {
    if chunk.desired_state == DesiredChunkState::ToUnload {
      if let Some(mesh_index) = chunk.mesh_index {
        vm_meshes.remove(mesh_index).unwrap();
      }
      false
    } else {
      match chunk.desired_state {
        DesiredChunkState::Loaded if matches!(chunk.current_state, CurrentChunkState::Rendered | CurrentChunkState::CalculatingMesh | CurrentChunkState::RecalculatingMesh) => {
          if let Some(mesh_index) = chunk.mesh_index {
            vm_meshes.remove(mesh_index).unwrap();
          }
          chunk.mesh_index = None;
          chunk.current_state = CurrentChunkState::Loaded;
        },
        _ => (),
      }
      true
    }
  })
}

fn start_required_tasks(
  task_manager: UniqueView<ChunkTaskManager>,
  mut world: UniqueViewMut<ChunkStorage>,
) {
  if !world.is_modified() {
    return
  }
  //HACK: cant iterate over chunks.keys() or chunk directly!
  let hashmap_keys: Vec<IVec3> = world.chunks.keys().copied().collect();
  for position in hashmap_keys {
    let chunk = world.chunks.get(&position).unwrap();
    match chunk.desired_state {
      DesiredChunkState::Loaded | DesiredChunkState::Rendered if chunk.current_state == CurrentChunkState::Nothing => {
        //start load task
        task_manager.spawn_task(ChunkTask::LoadChunk {
          seed: 0xbeef_face_dead_cafe,
          position
        });
        //Update chunk state
        let chunk = world.chunks.get_mut(&position).unwrap();
        chunk.current_state = CurrentChunkState::Loading;
        // ===========
        //log::trace!("Started loading chunk {position}");
      },
      DesiredChunkState::Rendered if (chunk.current_state == CurrentChunkState::Loaded || chunk.dirty) => {
        //get needed data
        let Some(neighbors) = world.neighbors_all(position) else {
          continue
        };
        let Some(data) = neighbors.mesh_data() else {
          continue
        };
        //spawn task
        task_manager.spawn_task(ChunkTask::GenerateMesh { data, position });
        //Update chunk state
        let chunk = world.chunks.get_mut(&position).unwrap();
        if chunk.dirty {
          chunk.current_state = CurrentChunkState::RecalculatingMesh;
        } else {
          chunk.current_state = CurrentChunkState::CalculatingMesh;
        }
        chunk.dirty = false;
        // ===========
        //log::trace!("Started generating mesh for chunk {position}");
      }
      _ => ()
    }
  }
}

fn process_completed_tasks(
  task_manager: UniqueView<ChunkTaskManager>,
  mut world: UniqueViewMut<ChunkStorage>,
  mut meshes: NonSendSync<UniqueViewMut<ChunkMeshStorage>>,
  renderer: NonSendSync<UniqueView<Renderer>>
) {
  for _ in 0..MAX_CHUNK_OPS {
    if let Some(res) = task_manager.receive() {
      match res {
        ChunkTaskResponse::LoadedChunk { position, chunk_data } => {
          //check if chunk exists
          let Some(chunk) = world.chunks.get_mut(&position) else {
            log::warn!("blocks data discarded: chunk doesn't exist");
            return
          };

          //check if chunk still wants it
          if !matches!(chunk.desired_state, DesiredChunkState::Loaded | DesiredChunkState::Rendered) {
            log::warn!("block data discarded: state undesirable: {:?}", chunk.desired_state);
            return
          }

          //set the block data
          chunk.block_data = Some(ChunkData {
            blocks: chunk_data
          });

          //update chunk state
          chunk.current_state = CurrentChunkState::Loaded;
        },
        ChunkTaskResponse::GeneratedMesh { position, vertices, indexes } => {
          //check if chunk exists
          let Some(chunk) = world.chunks.get_mut(&position) else {
            log::warn!("mesh discarded: chunk doesn't exist");
            return
          };

          //check if chunk still wants it
          if chunk.desired_state != DesiredChunkState::Rendered {
            log::warn!("mesh discarded: state undesirable: {:?}", chunk.desired_state);
            return
          }

          //apply the mesh
          let vertex_buffer = VertexBuffer::new(&renderer.display, &vertices).unwrap();
          let index_buffer = IndexBuffer::new(&renderer.display, PrimitiveType::TrianglesList, &indexes).unwrap();
          let mesh = ChunkMesh {
            vertex_buffer,
            index_buffer,
          };
          if let Some(index) = chunk.mesh_index {
            meshes.update(index, mesh).expect("Mesh update failed");
          } else {
            let mesh_index = meshes.insert(mesh);
            chunk.mesh_index = Some(mesh_index);
          }

          //update chunk state
          chunk.current_state = CurrentChunkState::Rendered;
        }
      }
    }
  }
}