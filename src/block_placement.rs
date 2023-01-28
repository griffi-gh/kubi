use glam::Vec3;
use shipyard::{UniqueViewMut, UniqueView, View, IntoIter};
use crate::{
  player::MainPlayer, 
  world::{raycast::LookingAtBlock, ChunkStorage, block::Block}, 
  input::Inputs
};

pub fn block_placement_system(
  main_player: View<MainPlayer>,
  raycast: View<LookingAtBlock>,
  input: UniqueView<Inputs>,
  mut world: UniqueViewMut<ChunkStorage>
) {
  //this cant process both place and break btw
  if input.action_a || input.action_b {
    //get raycast info
    let Some(ray) = (&main_player, &raycast).iter().next().unwrap().1/**/.0 else { return };
    //update block
    let is_place = input.action_b;
    let place_position = if is_place {
      let position = (ray.position - ray.direction * 0.5).floor().as_ivec3();
      let Some(block) = world.get_block_mut(position) else { return };
      *block = Block::Dirt;
      position
    } else {
      let Some(block) = world.get_block_mut(ray.block_position) else { return };
      *block = Block::Air;
      ray.block_position
    };
    //mark chunk as dirty
    let (chunk_pos, _) = ChunkStorage::to_chunk_coords(place_position);
    let chunk = world.chunks.get_mut(&chunk_pos).unwrap();
    chunk.dirty = true;
  }
}
