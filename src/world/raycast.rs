use glam::{Vec3, IVec3};
use shipyard::{View, Component, ViewMut, IntoIter, UniqueView, UniqueViewMut};
use crate::{player::MainPlayer, transform::Transform, input::Inputs};

use super::{ChunkStorage, block::Block};

const RAYCAST_STEP: f32 = 0.25;

#[derive(Clone, Copy, Debug)]
pub struct RaycastReport {
  pub length: f32,
  pub position: Vec3,
  pub block_position: IVec3,
  pub block: Block,
}

impl ChunkStorage {
  //this is probably pretty slow...
  pub fn raycast(&self, origin: Vec3, direction: Vec3, limit: Option<f32>) -> Option<RaycastReport> {
    debug_assert!(direction.is_normalized(), "Ray direction not normalized");
    let mut position = origin;
    let mut length = 0.;
    loop {
      let block_position = position.floor().as_ivec3();
      if let Some(block) = self.get_block(block_position) {
        if block.descriptor().raycast_collision {
          return Some(RaycastReport { length, position, block_position, block });
        }
      }
      length += RAYCAST_STEP;
      position += direction * RAYCAST_STEP;
      if let Some(limit) = limit {
        if length > limit {
          return None;
        }
      }
    }
  }
}

#[derive(Component, Clone, Copy, Debug, Default)]
pub struct LookingAtBlock(pub Option<RaycastReport>);

pub fn update_raycasts(
  transform: View<Transform>,
  mut raycast: ViewMut<LookingAtBlock>,
  world: UniqueView<ChunkStorage>,
) {
  //idk if this check is even needed
  if !(world.is_inserted_or_modified() || (transform.inserted_or_modified(), &raycast).iter().next().is_some()) {
    return
  }
  for (transform, report) in (&transform, &mut raycast).iter() {
    let (_, rotation, position) = transform.0.to_scale_rotation_translation();
    let direction = rotation * Vec3::NEG_Z;
    *report = LookingAtBlock(world.raycast(position, direction, Some(30.)));
  }
}

pub fn break_block_test_only(
  raycast: View<LookingAtBlock>,
  input: UniqueView<Inputs>,
  mut world: UniqueViewMut<ChunkStorage>
) {
  if input.action_a {
    //get raycast info
    let Some(ray) = raycast.iter().next().unwrap().0 else { return };
    //update block
    let Some(block) = world.get_block_mut(ray.block_position) else { return };
    *block = Block::Air;
    //mark chunk as dirty
    let (chunk_pos, _) = ChunkStorage::to_chunk_coords(ray.block_position);
    let chunk = world.chunks.get_mut(&chunk_pos).unwrap();
    chunk.dirty = true;
  }
}
