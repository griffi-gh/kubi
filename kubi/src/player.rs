use shipyard::{Component, AllStoragesViewMut, View, IntoIter};
use crate::{
  transform::Transform,
  camera::Camera, 
  fly_controller::FlyController, 
  world::raycast::LookingAtBlock, 
  block_placement::PlayerHolding,
};
pub use kubi_shared::player::Player;

#[derive(Component)]
pub struct MainPlayer;

pub fn spawn_player (
  mut storages: AllStoragesViewMut
) {
  log::info!("spawning player");
  let entity_id = storages.add_entity((
    Player,
    MainPlayer,
    Transform::default(),
    Camera::default(),
    FlyController,
    LookingAtBlock::default(),
    PlayerHolding::default(),
  ));
}
