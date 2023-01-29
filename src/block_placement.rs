use shipyard::{UniqueViewMut, UniqueView, View, IntoIter, ViewMut, EntitiesViewMut};
use crate::{
  player::MainPlayer, 
  world::{raycast::LookingAtBlock, ChunkStorage, block::Block}, 
  input::{Inputs, PrevInputs}, 
  events::{EventComponent, player_actions::PlayerActionEvent},
};

pub fn block_placement_system(
  main_player: View<MainPlayer>,
  raycast: View<LookingAtBlock>,
  input: UniqueView<Inputs>,
  prev_input: UniqueView<PrevInputs>,
  mut world: UniqueViewMut<ChunkStorage>,
  mut entities: EntitiesViewMut,
  mut events: ViewMut<EventComponent>,
  mut player_events: ViewMut<PlayerActionEvent>,
) {
  let action_place = input.action_b && !prev_input.0.action_b;
  let action_break = input.action_a && !prev_input.0.action_a;
  if action_place ^ action_break {
    //get raycast info
    let Some(ray) = (&main_player, &raycast).iter().next().unwrap().1/**/.0 else { return };
    //update block
    let (place_position, place_block) = if action_place {
      let position = (ray.position - ray.direction * 0.5).floor().as_ivec3();
      let Some(block) = world.get_block_mut(position) else { return };
      *block = Block::Dirt;
      (position, *block)
    } else {
      let Some(block) = world.get_block_mut(ray.block_position) else { return };
      *block = Block::Air;
      (ray.block_position, *block)
    };
    //mark chunk as dirty
    let (chunk_pos, _) = ChunkStorage::to_chunk_coords(place_position);
    let chunk = world.chunks.get_mut(&chunk_pos).unwrap();
    chunk.dirty = true;
    //send event
    entities.add_entity(
      (&mut events, &mut player_events), 
      (EventComponent, PlayerActionEvent::UpdatedBlock {
        position: place_position,
        block: place_block,
      })
    );
  }
}
