use glam::{Vec3, Mat4, Quat, EulerRot, Vec2};
use shipyard::{Component, View, ViewMut, IntoIter, UniqueView, Workload, IntoWorkload};
use std::f32::consts::PI;
use crate::{transform::Transform, input::Inputs, settings::GameSettings, DeltaTime};

#[derive(Component)]
pub struct FlyController;

pub fn update_controllers() -> Workload {
  (
    update_look,
    update_movement
  ).into_workload()
}

const MAX_PITCH: f32 = PI/2. - 0.001;

fn update_look(
  controllers: View<FlyController>,
  mut transforms: ViewMut<Transform>,
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
  controllers: View<FlyController>,
  mut transforms: ViewMut<Transform>,
  inputs: UniqueView<Inputs>,
  dt: UniqueView<DeltaTime>,
) {
  let movement = inputs.movement * 30. * dt.0.as_secs_f32();
  if movement == Vec2::ZERO { return }
  for (_, mut transform) in (&controllers, &mut transforms).iter() {
    let (scale, rotation, mut translation) = transform.0.to_scale_rotation_translation();
    translation += (rotation * Vec3::NEG_Z) * movement.y;
    translation += (rotation * Vec3::X) * movement.x;
    transform.0 = Mat4::from_scale_rotation_translation(scale, rotation, translation);
  }
}
