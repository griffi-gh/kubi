use shipyard::{View, UniqueViewMut, NonSendSync, IntoIter};
use crate::{player::Player, transform::Transform};
use super::GameWorld;

pub fn load_world_around_player(
  v_player: View<Player>,
  v_transform: View<Transform>,
  vm_world: NonSendSync<UniqueViewMut<GameWorld>>,
) {
  for (player, transform) in (&v_player, v_transform.inserted_or_modified()).iter() {

  }
}
