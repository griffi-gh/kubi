use glam::{Vec3, Mat4, Quat, EulerRot};
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
    let (scale, mut rotation, mut translation) = transform.0.to_scale_rotation_translation();
    let look = inputs.look * settings.mouse_sensitivity;

    //rotation *= Quat::from_axis_angle(Vec3::Y, look.x);

    //old way
    // rotation = rotation.normalize();
    // rotation *= Quat::from_euler(EulerRot::ZYX, 0., look.x, look.y).normalize();
    // rotation = rotation.normalize();

    // let direction = (rotation * Vec3::Z).normalize();
    // let camera_right = Vec3::Y.cross(direction).normalize();
    // let camera_up = direction.cross(camera_right);
    // rotation *= Quat::from_axis_angle(Vec3::Y, look.x);
    // rotation *= Quat::from_axis_angle(camera_right, look.y);
    
    //translation += (rotation * Vec3::X) / 4.;

    transform.0 = Mat4::from_scale_rotation_translation(scale, rotation, translation);
  }
}
