use glam::{Mat4, EulerRot, Quat};
use shipyard::{Component, View, ViewMut, IntoIter, UniqueView};
use crate::{transform::Transform, input::Inputs};

#[derive(Component)]
pub struct FlyController;

pub fn update_controllers(
  controllers: View<FlyController>,
  mut transforms: ViewMut<Transform>,
  inputs: UniqueView<Inputs>
) {
  for (_, mut transform) in (&controllers, &mut transforms).iter() {
    let (scale, mut rotation, translation) = transform.0.to_scale_rotation_translation();
    rotation *= Quat::from_euler(EulerRot::XYZ, 0., 0.001, 0.);
    transform.0 = Mat4::from_scale_rotation_translation(scale, rotation, translation);
  }
}
