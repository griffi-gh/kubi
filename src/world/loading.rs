use glam::ivec3;
use shipyard::{View, UniqueView, UniqueViewMut, NonSendSync, IntoIter, Workload, IntoWorkload};
use crate::{player::LocalPlayer, transform::Transform, settings::GameSettings};
use super::{GameWorld, chunk::{ChunkState, CHUNK_SIZE}};

pub fn load_world_around_player() -> Workload {
  (
    mark_chunks,
  ).into_workload()
}

pub fn mark_chunks(
  v_settings: UniqueView<GameSettings>,
  v_local_player: View<LocalPlayer>,
  v_transform: View<Transform>,
  mut vm_world: NonSendSync<UniqueViewMut<GameWorld>>,
) {
  //Read game settings
  let load_distance = (v_settings.render_distance + 1) as i32;
  
  //Check if a player moved
  let Some((_, transform)) = (&v_local_player, v_transform.inserted_or_modified()).iter().next() else {
    return
  };

  //If it did, get it's position and current chunk
  let position = transform.0.to_scale_rotation_translation().2;
  let at_chunk = position.as_ivec3() / CHUNK_SIZE as i32;

  //Then, mark *ALL* chunks with ToUnload
  for (_, chunk) in &mut vm_world.chunks {
    chunk.desired_state = ChunkState::ToUnload;
  }

  //Then mark chunks that are near to the player
  for x in -load_distance..=load_distance {
    for y in -load_distance..=load_distance {
      for z in -load_distance..=load_distance {
        let chunk_pos_offset = ivec3(x, y, z);
        let is_border = chunk_pos_offset.to_array()
          .iter().any(|x| x.abs() == load_distance);
        let desired = match is_border {
          true  => ChunkState::Loaded,
          false => ChunkState::Rendered,
        };
      }
    }
  }

  //TODO
  todo!()
}
