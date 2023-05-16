use shipyard::{Component, View, ViewMut, EntitiesViewMut, IntoIter, track};
use glam::{IVec3, Quat, Vec3};
use kubi_shared::block::Block;
use crate::{
  player::MainPlayer, 
  transform::Transform
};
use super::EventComponent;

#[derive(Component, Clone, Copy, Debug)]
pub enum PlayerActionEvent {
  PositionChanged {
    position: Vec3,
    direction: Quat
  },
  UpdatedBlock {
    position: IVec3,
    block: Block,
  },
}

pub fn generate_move_events(
  transforms: View<Transform, track::All>,
  player: View<MainPlayer>,
  mut entities: EntitiesViewMut,
  mut events: ViewMut<EventComponent>,
  mut actions: ViewMut<PlayerActionEvent>,
) {
  let Some((_, transform)) = (&player, transforms.inserted_or_modified()).iter().next() else { return };
  let (_, direction, position) = transform.0.to_scale_rotation_translation();
  entities.add_entity(
    (&mut events, &mut actions), 
    (EventComponent, PlayerActionEvent::PositionChanged { position, direction })
  );
}
