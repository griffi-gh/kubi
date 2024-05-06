use shipyard::{AllStoragesView, AllStoragesViewMut, IntoIter, IntoWorkload, Unique, UniqueView, UniqueViewMut, View, Workload};
use winit::dpi::PhysicalSize;
use glam::{mat4, vec4, Mat4, UVec2, Vec3};
use crate::{events::WindowResizedEvent, state::is_ingame};

mod renderer;
pub use renderer::Renderer;

use self::camera::update_camera_unform_buffer;

pub mod world;
pub mod camera;
//pub mod primitives;
//pub mod selection_box;
//pub mod entities;
//pub mod sumberge;

// pub const WGPU_COORDINATE_SYSTEM: Mat4 = mat4(
//   vec4(1.0, 0.0, 0.0, 0.0),
//   vec4(0.0, 1.0, 0.0, 0.0),
//   vec4(0.0, 0.0, 0.5, 0.5),
//   vec4(0.0, 0.0, 0.0, 1.0),
// );

// #[repr(C)]
// #[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
// struct TrasnformUniformData {
//  pub transform: [[f32; 4]; 4],
// }

// impl TrasnformUniformData {
//   pub const LAYOUT: &wgpu::Layou
// }

// impl From<Mat4> for TrasnformUniformData {
//   fn from(mat: Mat4) -> Self {
//     Self {
//       transform: mat.to_cols_array_2d(),
//     }
//   }
// }

// impl From<Transform> for TrasnformUniformData {
//   fn from(value: Transform) -> Self {
//     value.0.into()
//   }
// }

pub struct BufferPair {
  pub index: wgpu::Buffer,
  pub index_len: u32,
  pub vertex: wgpu::Buffer,
  pub vertex_len: u32,
}

#[derive(Unique)]
pub struct BackgroundColor(pub Vec3);

pub struct RenderCtx<'a> {
  pub renderer: &'a Renderer,
  pub encoder: &'a mut wgpu::CommandEncoder,
  pub surface_view: &'a wgpu::TextureView,
}

pub fn init_render_states() -> Workload {
  (
    camera::init_camera_uniform_buffer,
    world::init_world_render_state,
  ).into_workload()
}

pub fn render_master(storages: AllStoragesViewMut) {
  let renderer = storages.borrow::<UniqueView<Renderer>>().unwrap();

  let mut encoder = renderer.device().create_command_encoder(&wgpu::CommandEncoderDescriptor {
    label: Some("main_encoder"),
  });
  let surface_texture = renderer.surface().get_current_texture().unwrap();
  let surface_view = surface_texture.texture.create_view(&wgpu::TextureViewDescriptor::default());

  {
    let bg = storages.borrow::<UniqueView<BackgroundColor>>().unwrap().0;
    let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
      label: Some("rpass_background"),
      color_attachments: &[Some(wgpu::RenderPassColorAttachment {
        view: &surface_view,
        resolve_target: None,
        ops: wgpu::Operations {
          load: wgpu::LoadOp::Clear(wgpu::Color {
            r: bg.x as f64,
            g: bg.y as f64,
            b: bg.z as f64,
            a: 1.0,
          }),
          store: wgpu::StoreOp::Store,
        },
      })],
      depth_stencil_attachment: None,
      ..Default::default()
    });
  }

  let mut data = RenderCtx {
    renderer: &renderer,
    encoder: &mut encoder,
    surface_view: &surface_view,
  };

  if storages.run(is_ingame) {
    //XXX: probably should be in pre_update or sth
    storages.run(update_camera_unform_buffer);

    //TODO init world render state on demand
    storages.run_with_data(world::draw_world, &mut data);
  }

  renderer.queue().submit(std::iter::once(encoder.finish()));
  surface_texture.present();
}

/// Resize the renderer when the window is resized
pub fn resize_renderer(
  mut renderer: UniqueViewMut<Renderer>,
  resize: View<WindowResizedEvent>,
) {
  if let Some(size) = resize.iter().last() {
    renderer.resize(PhysicalSize::new(size.0.x, size.0.y));
  }
}

//Deprecated WindowSize thingy

#[derive(Unique, Clone, Copy)]
#[repr(transparent)]
#[deprecated = "use Renderer.size instead"]
#[allow(deprecated)]
pub struct WindowSize(pub UVec2);

pub fn init_window_size(storages: AllStoragesView) {
  let size = storages.borrow::<View<WindowResizedEvent>>().unwrap().iter().next().unwrap().0;
  storages.add_unique(WindowSize(size))
}

pub fn update_window_size(
  mut win_size: UniqueViewMut<WindowSize>,
  resize: View<WindowResizedEvent>,
) {
  if let Some(resize) = resize.iter().next() {
    win_size.0 = resize.0;
  }
}

// pub fn if_resized (
//   resize: View<WindowResizedEvent>,
// ) -> bool {
//   resize.len() > 0
// }
