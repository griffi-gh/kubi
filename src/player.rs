use glam::Mat4;
use shipyard::{Component, AllStoragesViewMut};
use crate::{
  transform::Transform,
  camera::Camera, 
  fly_controller::FlyController, 
  world::raycast::LookingAtBlock,
};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct MainPlayer;

pub fn spawn_player (
  mut storages: AllStoragesViewMut
) {
  log::info!("spawning player");
  storages.add_entity((
    Player,
    MainPlayer,
    Transform::default(),
    Camera::default(),
    FlyController,
    LookingAtBlock::default(),
  ));
}
