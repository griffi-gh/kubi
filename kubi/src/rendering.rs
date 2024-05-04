use shipyard::{AllStoragesView, AllStoragesViewMut, IntoIter, Unique, UniqueView, UniqueViewMut, View};
use winit::dpi::PhysicalSize;
use glam::{Vec3, UVec2};
use crate::{events::WindowResizedEvent, state::is_ingame};

pub mod renderer;
pub mod primitives;
pub mod world;
pub mod selection_box;
pub mod entities;
pub mod sumberge;

pub use renderer::Renderer;
pub struct BufferPair {
  pub index: wgpu::Buffer,
  pub vertex: wgpu::Buffer,
}

#[derive(Unique)]
#[repr(transparent)]
pub struct BackgroundColor(pub Vec3);

#[derive(Unique, Clone, Copy)]
#[repr(transparent)]
#[deprecated = "use Renderer.size instead"]
#[allow(deprecated)]
pub struct WindowSize(pub UVec2);

pub fn render_master(storages: AllStoragesViewMut) {
  let renderer = storages.borrow::<UniqueView<Renderer>>().unwrap();

  let mut encoder = renderer.device().create_command_encoder(&wgpu::CommandEncoderDescriptor {
    label: Some("main_encoder"),
  });
  let surface_texture = renderer.surface().get_current_texture().unwrap();
  let surface_view = surface_texture.texture.create_view(&wgpu::TextureViewDescriptor::default());

  //Main in-game render pass
  if storages.run(is_ingame) {
    let bg = storages.borrow::<UniqueView<BackgroundColor>>().unwrap().0;
    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
      label: Some("main0_pass"),
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

    let data = (&mut render_pass, &*renderer);

    storages.run_with_data(world::draw_world, data);
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
