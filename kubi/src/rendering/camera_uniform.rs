use shipyard::{Unique, UniqueView, AllStoragesView, View, IntoIter};
use wgpu::util::DeviceExt;
use crate::camera::Camera;
use super::{Renderer, OPENGL_TO_WGPU_MATRIX};

#[repr(C, packed)]
#[derive(Clone, Copy, Default, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
  view_proj: [[f32; 4]; 4],
}

#[derive(Unique)]
pub struct CameraUniformBuffer(pub wgpu::Buffer);

pub fn init_camera_uniform(
  storages: AllStoragesView
) {
  let buffer = {
    let renderer = storages.borrow::<UniqueView<Renderer>>().unwrap();
    CameraUniformBuffer(
      renderer.device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
          label: Some("CameraUniformBuffer"),
          contents: bytemuck::cast_slice(&[CameraUniform::default()]),
          usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        }
      )
    )
  };
  storages.add_unique(buffer);
}

pub fn update_camera_uniform(
  renderer: UniqueView<Renderer>,
  camera: View<Camera>,
  buffer: UniqueView<CameraUniformBuffer>,
) {
  //Just pick the first camera, if it exists, of course
  let Some(camera) = camera.iter().next() else { return };
  renderer.queue.write_buffer(&buffer.0, 0, bytemuck::cast_slice(&[
    CameraUniform {
      view_proj: (camera.perspective_matrix * camera.view_matrix * OPENGL_TO_WGPU_MATRIX).to_cols_array_2d()
    }
  ]));
}
