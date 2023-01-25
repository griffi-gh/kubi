use glam::{Vec3, Mat4, EulerRot, Quat};
use shipyard::{Component, View, ViewMut, IntoIter, UniqueView};
use crate::{transform::Transform, input::Inputs, settings::GameSettings};

#[derive(Component)]
pub struct FlyController;

pub fn update_controllers(
  controllers: View<FlyController>,
  mut transforms: ViewMut<Transform>,
  inputs: UniqueView<Inputs>,
  settings: UniqueView<GameSettings>,
) {
  for (_, mut transform) in (&controllers, &mut transforms).iter() {
    let (scale, mut rotation, translation) = transform.0.to_scale_rotation_translation();
    let look = inputs.look * settings.mouse_sensitivity * -1.;
    rotation *= Quat::from_euler(EulerRot::YXZ, look.x, look.y, 0.);
    transform.0 = Mat4::from_scale_rotation_translation(scale, rotation, translation);
  }
}
