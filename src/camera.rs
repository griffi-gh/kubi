use glam::{Mat4, Vec3, EulerRot};
use shipyard::{Component, ViewMut, View, IntoIter, Workload, IntoWorkload};
use std::f32::consts::PI;
use crate::{transform::Transform, events::WindowResizedEvent};

#[derive(Component)]
pub struct Camera {
  pub view_matrix: Mat4,
  pub perspective_matrix: Mat4,
  pub up: Vec3,
  pub fov: f32,
  pub z_near: f32,
  pub z_far: f32,
}
impl Camera {
  pub fn new(fov: f32, z_near: f32, z_far: f32, up: Vec3) -> Self {
    Self {
      fov, z_near, z_far, up,
      perspective_matrix: Mat4::default(),
      view_matrix: Mat4::default(),
    }
  }
}
impl Default for Camera {
  fn default() -> Self {
    Self::new(PI / 3., 0.1, 1024., Vec3::Y)
  }
}

pub fn compute_cameras() -> Workload {
  (
    update_perspective_matrix,
    update_view_matrix,
  ).into_workload()
}

fn update_view_matrix(
  mut vm_camera: ViewMut<Camera>,
  v_transform: View<Transform>
) {
  for (camera, transform) in (&mut vm_camera, v_transform.inserted_or_modified()).iter() {
    let (_, rotation, translation) = transform.0.to_scale_rotation_translation();
    let direction = rotation * Vec3::NEG_Z;
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
  for camera in (&mut vm_camera).iter() {
    camera.perspective_matrix = Mat4::perspective_rh_gl(
      camera.fov, 
      size.0.x as f32 / size.0.y as f32, 
      camera.z_near,
      camera.z_far, 
    )
  }
}
