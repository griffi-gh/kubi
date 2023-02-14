use glam::{Vec3, Mat4};
use shipyard::{ViewMut, View, IntoIter, Workload, IntoWorkload, track};
use crate::{transform::Transform, events::WindowResizedEvent};
use super::Camera;

//maybe parallelize these two?

fn update_view_matrix(
  mut vm_camera: ViewMut<Camera>,
  v_transform: View<Transform, { track::All }>
) {
  for (mut camera, transform) in (&mut vm_camera, v_transform.inserted_or_modified()).iter() {
    let (_, rotation, translation) = transform.0.to_scale_rotation_translation();
    let direction = (rotation.normalize() * Vec3::NEG_Z).normalize();
    camera.view_matrix = Mat4::look_to_rh(translation, direction, camera.up);
  }
}

fn update_perspective_matrix(
  mut vm_camera: ViewMut<Camera>,
  resize: View<WindowResizedEvent>,
) {
  //TODO update on launch
  let Some(&size) = resize.iter().next() else {
    return
  };
  for mut camera in (&mut vm_camera).iter() {
    camera.perspective_matrix = Mat4::perspective_rh_gl(
      camera.fov, 
      size.0.x as f32 / size.0.y as f32, 
      camera.z_near,
      camera.z_far, 
    )
  }
}

pub fn update_matrices() -> Workload {
  (
    update_view_matrix, 
    update_perspective_matrix,
  ).into_workload()
}
