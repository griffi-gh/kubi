use glam::ivec3;
use shipyard::{View, UniqueView, UniqueViewMut, IntoIter, Workload, IntoWorkload, NonSendSync};
use crate::{player::LocalPlayer, transform::Transform, settings::GameSettings};
use super::{ChunkStorage, chunk::{Chunk, ChunkState, CHUNK_SIZE}, ChunkMeshStorage};

pub fn load_world_around_player() -> Workload {
  (
    update_chunks_if_player_moved,
    unload_marked_chunks
  ).into_workload()
}

pub fn update_chunks_if_player_moved(
  v_settings: UniqueView<GameSettings>,
  v_local_player: View<LocalPlayer>,
  v_transform: View<Transform>,
  mut vm_world: UniqueViewMut<ChunkStorage>,
) {
  //Check if the player actually moved
  let Some((_, transform)) = (&v_local_player, v_transform.inserted_or_modified()).iter().next() else {
    return
  };

  //Read game settings
  let load_distance = (v_settings.render_distance + 1) as i32;

  //If it did, get it's position and current chunk
  let player_position = transform.0.to_scale_rotation_translation().2;
  let player_at_chunk = player_position.as_ivec3() / CHUNK_SIZE as i32;

  //Then, mark *ALL* chunks with ToUnload
  for (_, chunk) in &mut vm_world.chunks {
    chunk.desired_state = ChunkState::ToUnload;
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
          true  => ChunkState::Loaded,
          false => ChunkState::Rendered,
        };
        chunk.desired_state = desired;
      }
    }
  }
}

fn unload_marked_chunks(
  mut vm_world: UniqueViewMut<ChunkStorage>,
  mut vm_meshes: NonSendSync<UniqueViewMut<ChunkMeshStorage>>
) {
  if !vm_world.is_modified() {
    return
  }
  vm_world.chunks.retain(|_, chunk| {
    if chunk.desired_state == ChunkState::ToUnload {
      if let Some(mesh_index) = chunk.mesh_index {
        vm_meshes.remove(mesh_index).unwrap();
      }
      false
    } else {
      true
    }
  })
}
