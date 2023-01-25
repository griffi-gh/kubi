use glam::Mat4;
use shipyard::{Component, EntitiesViewMut, ViewMut};
use crate::{
  transform::Transform,
  camera::Camera, 
  fly_controller::FlyController,
};

#[derive(Component)]
pub struct LocalPlayer;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct MainPlayer;

pub fn spawn_player (
  mut entities: EntitiesViewMut,
  mut vm_player: ViewMut<Player>,
  mut vm_main_player: ViewMut<MainPlayer>,
  mut vm_local_player: ViewMut<LocalPlayer>,
  mut vm_transform: ViewMut<Transform>,
  mut vm_camera: ViewMut<Camera>,
  mut vm_controls: ViewMut<FlyController>,
) {
  log::info!("spawning player");
  entities.add_entity(
    (
      &mut vm_player,
      &mut vm_main_player,
      &mut vm_local_player,
      &mut vm_transform,
      &mut vm_camera,
      &mut vm_controls
    ),
    (
      Player,
      MainPlayer,
      LocalPlayer,
      Transform(Mat4::default()),
      Camera::default(),
      FlyController
    )
  );
}
