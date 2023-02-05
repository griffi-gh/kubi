use shipyard::{UniqueViewMut, UniqueView, View, IntoIter, ViewMut, EntitiesViewMut};
use crate::{
  player::MainPlayer, 
  world::{raycast::LookingAtBlock, ChunkStorage, block::Block, queue::{BlockUpdateQueue, BlockUpdateEvent}}, 
  input::{Inputs, PrevInputs}, 
  events::{EventComponent, player_actions::PlayerActionEvent},
};

pub fn block_placement_system(
  main_player: View<MainPlayer>,
  raycast: View<LookingAtBlock>,
  input: UniqueView<Inputs>,
  prev_input: UniqueView<PrevInputs>,
  mut block_event_queue: UniqueViewMut<BlockUpdateQueue>,
  mut entities: EntitiesViewMut,
  mut events: ViewMut<EventComponent>,
  mut player_events: ViewMut<PlayerActionEvent>,
) {
  let action_place = input.action_b && !prev_input.0.action_b;
  let action_break = input.action_a && !prev_input.0.action_a;
  if action_place ^ action_break {
    //get raycast info
    let Some(ray) = (&main_player, &raycast).iter().next().unwrap().1/**/.0 else { return };
    //get coord and block type
    let (place_position, place_block) = if action_place {
      let position = (ray.position - ray.direction * 0.5).floor().as_ivec3();
      (position, Block::Cobblestone)
    } else {
      (ray.block_position, Block::Air)
    };
    //queue place
    block_event_queue.push(BlockUpdateEvent {
      position: place_position,
      value: place_block,
    });
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
