use glam::Mat4;
use shipyard::{Component, EntitiesViewMut, ViewMut};

use crate::transform::Transform;

#[derive(Component)]
pub struct Player;

pub fn spawn_player (
  mut entities: EntitiesViewMut,
  mut vm_player: ViewMut<Player>,
  mut vm_transform: ViewMut<Transform>
) {
  log::info!("spawning player");
  entities.add_entity(
    (
      &mut vm_player,
      &mut vm_transform
    ),
    (
      Player,
      Transform(Mat4::default())
    )
  );
}
