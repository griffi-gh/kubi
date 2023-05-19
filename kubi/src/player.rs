use shipyard::{Component, AllStoragesViewMut};
use kubi_shared::{
  entity::{Entity, Health}, 
  player::{PLAYER_HEALTH, PlayerHolding}, 
  networking::client::Username
};
use crate::{
  transform::Transform,
  camera::Camera, 
  fly_controller::FlyController, 
  world::raycast::LookingAtBlock, 
};
pub use kubi_shared::player::Player;

#[derive(Component)]
pub struct MainPlayer;

pub fn spawn_player (
  mut storages: AllStoragesViewMut
) {
  log::info!("spawning player");
  storages.add_entity((
    Player,
    MainPlayer,
    Entity,
    Health::new(PLAYER_HEALTH),
    Transform::default(),
    Camera::default(),
    FlyController,
    LookingAtBlock::default(),
    PlayerHolding::default(),
    Username("Sbeve".into())
  ));
}
