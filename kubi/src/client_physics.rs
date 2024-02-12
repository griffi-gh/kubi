//TODO client-side physics
//TODO move this to shared
use glam::{Mat4, Vec3};
use kubi_shared::transform::Transform;
use shipyard::{track, AllStoragesView, Component, IntoIter, Unique, UniqueView, View, ViewMut};
use crate::delta_time::DeltaTime;

#[derive(Unique)]
pub struct GlobalClPhysicsConfig {
  pub gravity: Vec3,
}

#[derive(Component)]
pub struct ClPhysicsActor {
  pub forces: Vec3,
  pub velocity: Vec3,
  pub terminal_velocity: f32,
  //TODO: this should be configurable per block
  pub friction_agains_ground: f32,
}

impl Default for ClPhysicsActor {
  fn default() -> Self {
    Self {
      forces: Vec3::ZERO,
      velocity: Vec3::ZERO,
      terminal_velocity: 40.,
      friction_agains_ground: 0.5,
    }
  }
}

pub fn init_client_physics(
  storages: AllStoragesView,
) {
  storages.add_unique(GlobalClPhysicsConfig {
    gravity: Vec3::new(0., -9.8, 0.),
  });
}

pub fn update_client_physics_late(
  controllers: View<ClPhysicsActor>,
  mut transforms: ViewMut<Transform, track::All>,
  dt: UniqueView<DeltaTime>,
  phy_conf: UniqueView<GlobalClPhysicsConfig>,
) {
  // for (_, mut transform) in (&controllers, &mut transforms).iter() {
  //   let (scale, rotation, mut translation) = transform.0.to_scale_rotation_translation();
  //   translation.y -= dt.0.as_secs_f32() * 100.;
  //   transform.0 = Mat4::from_scale_rotation_translation(scale, rotation, translation);
  // }
}
