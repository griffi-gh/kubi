use glam::{Vec3, Mat4};
use shipyard::{ViewMut, View, IntoIter, Workload, IntoWorkload, track, UniqueView, SystemModificator};
use crate::{transform::Transform, events::WindowResizedEvent, rendering::Renderer};
use super::Camera;

//maybe parallelize these two?

fn update_view_matrix(
  mut vm_camera: ViewMut<Camera>,
  v_transform: View<Transform, track::All>
) {
  for (mut camera, transform) in (&mut vm_camera, v_transform.inserted_or_modified()).iter() {
    let (_, rotation, translation) = transform.0.to_scale_rotation_translation();
    let direction = (rotation.normalize() * Vec3::NEG_Z).normalize();
    camera.view_matrix = Mat4::look_to_rh(translation, direction, camera.up);
  }
}

fn update_perspective_matrix(
  mut vm_camera: ViewMut<Camera>,
  renderer: UniqueView<Renderer>
) {
  let size = renderer.size;
  for mut camera in (&mut vm_camera).iter() {
    camera.perspective_matrix = Mat4::perspective_rh_gl(
      camera.fov,
      size.width as f32 / size.height as f32,
      camera.z_near, camera.z_far
    )
  }
}

fn need_perspective_calc(
  v_camera: View<Camera>,
  resize_event: View<WindowResizedEvent>,
) -> bool {
  (resize_event.len() > 0) ||
  (v_camera.iter().any(|camera| {
    camera.perspective_matrix == Mat4::default()
  }))
}

pub fn update_matrices() -> Workload {
  (
    update_view_matrix, 
    update_perspective_matrix.run_if(need_perspective_calc),
  ).into_sequential_workload()
}
