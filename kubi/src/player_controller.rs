use glam::{Vec3, Mat4, Quat, EulerRot, Vec2};
use shipyard::{Component, View, ViewMut, IntoIter, UniqueView, Workload, IntoWorkload, track};
use std::f32::consts::PI;
use crate::{transform::Transform, input::Inputs, settings::GameSettings, delta_time::DeltaTime};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlayerControllerType {
  FlyCam,
  FpsCtl,
}

#[derive(Component)]
pub struct PlayerController {
  pub control_type: PlayerControllerType,
  pub speed: f32,
}

impl PlayerController {
  pub const DEFAULT_FLY_CAM: Self = Self {
    control_type: PlayerControllerType::FlyCam,
    speed: 30.,
  };

  pub const DEFAULT_FPS_CTL: Self = Self {
    control_type: PlayerControllerType::FpsCtl,
    speed: 10.,
  };
}

pub fn update_player_controllers() -> Workload {
  (
    update_look,
    update_movement
  ).into_sequential_workload()
}

const MAX_PITCH: f32 = PI/2. - 0.05;

fn update_look(
  controllers: View<PlayerController>,
  mut transforms: ViewMut<Transform, track::All>,
  inputs: UniqueView<Inputs>,
  settings: UniqueView<GameSettings>,
  dt: UniqueView<DeltaTime>,
) {
  let look = inputs.look * settings.mouse_sensitivity * dt.0.as_secs_f32();
  if look == Vec2::ZERO { return }
  for (_, mut transform) in (&controllers, &mut transforms).iter() {
    let (scale, mut rotation, translation) = transform.0.to_scale_rotation_translation();
    let (mut yaw, mut pitch, _roll) = rotation.to_euler(EulerRot::YXZ);
    yaw -= look.x;
    pitch -= look.y;
    pitch = pitch.clamp(-MAX_PITCH, MAX_PITCH);
    rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.).normalize();
    transform.0 = Mat4::from_scale_rotation_translation(scale, rotation, translation);
  }
}

fn update_movement(
  controllers: View<PlayerController>,
  mut transforms: ViewMut<Transform, track::All>,
  inputs: UniqueView<Inputs>,
  dt: UniqueView<DeltaTime>,
) {
  if inputs.movement == Vec2::ZERO { return }
  let movement = inputs.movement * dt.0.as_secs_f32();
  for (_, mut transform) in (&controllers, &mut transforms).iter() {
    let (scale, rotation, mut translation) = transform.0.to_scale_rotation_translation();
    let rotation_norm = rotation.normalize();
    translation += (rotation_norm * Vec3::NEG_Z).normalize() * movement.y;
    translation += (rotation_norm * Vec3::X).normalize() * movement.x;
    transform.0 = Mat4::from_scale_rotation_translation(scale, rotation_norm, translation);
  }
}
