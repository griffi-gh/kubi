use glam::{vec3, EulerRot, Mat4, Quat, Vec2, Vec2Swizzles, Vec3, Vec3Swizzles};
use shipyard::{track, Component, Get, IntoIter, IntoWithId, IntoWorkload, Unique, UniqueView, View, ViewMut, Workload};
use winit::keyboard::KeyCode;
use std::f32::consts::PI;
use crate::{client_physics::ClPhysicsActor, delta_time::DeltaTime, input::{Inputs, RawKbmInputState}, settings::GameSettings, transform::Transform};

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
    speed: 50.,
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
  mut actors: ViewMut<ClPhysicsActor>,
  inputs: UniqueView<Inputs>,
  dt: UniqueView<DeltaTime>,
) {
  if (inputs.movement == Vec2::ZERO) && !inputs.jump { return }
  let movement = inputs.movement.extend(inputs.jump as u32 as f32).xzy();
  for (id, (ctl, mut transform)) in (&controllers, &mut transforms).iter().with_id() {
    let (scale, rotation, mut translation) = transform.0.to_scale_rotation_translation();
    let rotation_norm = rotation.normalize();
    match ctl.control_type {
      PlayerControllerType::FlyCam => {
        translation += (rotation_norm * Vec3::NEG_Z).normalize() * movement.z * ctl.speed * dt.0.as_secs_f32();
        translation += (rotation_norm * Vec3::X).normalize() * movement.x * ctl.speed * dt.0.as_secs_f32();
        translation += Vec3::Y * movement.y * ctl.speed * dt.0.as_secs_f32();
        transform.0 = Mat4::from_scale_rotation_translation(scale, rotation_norm, translation);
      },
      PlayerControllerType::FpsCtl => {
        let mut actor = (&mut actors).get(id).unwrap();

        let euler = rotation_norm.to_euler(EulerRot::YZX);
        let right = Vec2::from_angle(-euler.0).extend(0.).xzy();
        let forward = Vec2::from_angle(-(euler.0 + PI/2.)).extend(0.).xzy();

        actor.apply_force(ctl.speed * (
          (forward * movement.z) +
          (right * movement.x) +
          (Vec3::Y * movement.y)
        ));

        // translation += forward * movement.z * ctl.speed * dt.0.as_secs_f32();
        // translation += right * movement.x * ctl.speed * dt.0.as_secs_f32();
        // translation += Vec3::Y * movement.y * ctl.speed * dt.0.as_secs_f32();

        // transform.0 = Mat4::from_scale_rotation_translation(scale, rotation_norm, translation);
      }
    }
  }
}

pub fn debug_switch_ctl_type(
  mut controllers: ViewMut<PlayerController>,
  mut actors: ViewMut<ClPhysicsActor>,
  kbm_state: UniqueView<RawKbmInputState>,
) {
  for (mut controller, mut actor) in (&mut controllers, &mut actors).iter() {
    if kbm_state.keyboard_state.contains(KeyCode::F4 as u32) {
      *controller = PlayerController::DEFAULT_FPS_CTL;
      actor.disable = false;
    } else if kbm_state.keyboard_state.contains(KeyCode::F5 as u32) {
      *controller = PlayerController::DEFAULT_FLY_CAM;
      actor.disable = true;
    }
  }
}
