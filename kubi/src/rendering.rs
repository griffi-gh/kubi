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

  if storages.run(is_ingame) {
    let bg_color = storages.borrow::<UniqueView<BackgroundColor>>().unwrap();
    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
      label: Some("main0_pass"),
      color_attachments: &[Some(wgpu::RenderPassColorAttachment {
        view: &surface_view,
        resolve_target: None,
        ops: wgpu::Operations {
          load: wgpu::LoadOp::Clear(wgpu::Color {
            r: bg_color.0.x as f64,
            g: bg_color.0.y as f64,
            b: bg_color.0.z as f64,
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

    // render_pass.set_pipeline(&renderer.pipeline);
    // render_pass.set_bind_group(0, &renderer.bind_group, &[]);
    // render_pass.set_vertex_buffer(0, renderer.vertex_buffer.slice(..));
    // render_pass.set_index_buffer(renderer.index_buffer.slice(..));
    // render_pass.draw_indexed(0..renderer.num_indices, 0, 0..1);
  }

  renderer.queue().submit(std::iter::once(encoder.finish()));
  surface_texture.present();
}

// pub fn clear_background(
//   mut target: NonSendSync<UniqueViewMut<RenderTarget>>,
//   color: UniqueView<BackgroundColor>,
// ) {
//   target.0.clear_color_srgb_and_depth((color.0.x, color.0.y, color.0.z, 1.), 1.);
// }

//Resize the renderer

pub fn resize_renderer(
  mut renderer: UniqueViewMut<Renderer>,
  resize: View<WindowResizedEvent>,
) {
  if let Some(size) = resize.iter().last() {
    renderer.resize(PhysicalSize::new(size.0.x, size.0.y));
  }
}

//not sure if this belongs here

pub fn init_window_size(
  storages: AllStoragesView,
) {
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

pub fn if_resized (
  resize: View<WindowResizedEvent>,
) -> bool {
  resize.len() > 0
}
