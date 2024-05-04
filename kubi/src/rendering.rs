use pollster::FutureExt;
use raw_window_handle::HasRawWindowHandle;
use shipyard::{AllStoragesView, AllStoragesViewMut, IntoIter, NonSendSync, Unique, UniqueView, UniqueViewMut, View};
use wgpu::SurfaceTargetUnsafe;
use winit::{
  event_loop::ActiveEventLoop,
  window::{WindowAttributes, Fullscreen, Window},
  dpi::PhysicalSize
};
use glam::{Vec3, UVec2};
use crate::{events::WindowResizedEvent, settings::{GameSettings, FullscreenMode}};

pub mod primitives;
pub mod world;
pub mod selection_box;
pub mod entities;
pub mod sumberge;

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

#[derive(Unique)]
pub struct Renderer {
  instance: wgpu::Instance,
  surface: wgpu::Surface<'static>,
  device: wgpu::Device,
  queue: wgpu::Queue,
  surface_config: wgpu::SurfaceConfiguration,
  size: PhysicalSize<u32>,
  // pub depth_texture: wgpu::Texture,

  //must be last due to drop order
  window: Window,
}

impl Renderer {
  pub fn init(event_loop: &ActiveEventLoop, settings: &GameSettings) -> Self {
    log::info!("initializing display");

    let window_attributes = Window::default_attributes()
      .with_title("kubi")
      .with_maximized(true)
      .with_min_inner_size(PhysicalSize::new(640, 480))
      .with_fullscreen({
        //this has no effect on android, so skip this pointless stuff
        #[cfg(target_os = "android")] {
          None
        }
        #[cfg(not(target_os = "android"))]
        if let Some(fs_settings) = &settings.fullscreen {
          let monitor = event_loop.primary_monitor().or_else(|| {
            event_loop.available_monitors().next()
          });

          if let Some(monitor) = monitor {
            log::info!("monitor: {}", monitor.name().unwrap_or_else(|| "generic".into()));
            match fs_settings.mode {
              FullscreenMode::Borderless => {
                log::info!("starting in borderless fullscreen mode");
                Some(Fullscreen::Borderless(Some(monitor)))
              },
              FullscreenMode::Exclusive => {
                log::warn!("exclusive fullscreen mode is experimental");
                log::info!("starting in exclusive fullscreen mode");
                //TODO: grabbing the first video mode is probably not the best idea...
                monitor.video_modes().next()
                  .map(|vmode| {
                    log::info!("video mode: {}", vmode.to_string());
                    Some(Fullscreen::Exclusive(vmode))
                  })
                  .unwrap_or_else(|| {
                    log::warn!("no valid video modes found, falling back to windowed mode instead");
                    None
                  })
              }
            }
          } else {
            log::warn!("no monitors found, falling back to windowed mode");
            None
          }
        } else {
          log::info!("starting in windowed mode");
          None
        }
      });
    let window = event_loop.create_window(window_attributes).unwrap();

    let size = window.inner_size();

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
      backends: wgpu::Backends::all(),
      ..Default::default()
    });

    // Create a surface with `create_surface_unsafe` to get a surface with 'static lifetime
    // It should never outlive the window it's created from
    let surface = unsafe {
      instance.create_surface_unsafe(SurfaceTargetUnsafe::from_window(&window).unwrap()).unwrap()
    };

    let adapter = instance.request_adapter(
      &wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
      },
    ).block_on().unwrap();

    let (device, queue) = adapter.request_device(
      &wgpu::DeviceDescriptor {
        label: None,
        required_features: wgpu::Features::empty(),
        required_limits: wgpu::Limits::downlevel_defaults(),
      },
      None,
    ).block_on().unwrap();

    let surface_config = surface.get_default_config(&adapter, size.width, size.height).unwrap();
    surface.configure(&device, &surface_config);

    Self { window, instance, surface, device, queue, surface_config, size }
  }

  pub fn resize(&mut self, size: PhysicalSize<u32>) {
    if size.width == 0 || size.height == 0 {
      log::warn!("Ignoring resize event with zero width or height");
      return
    }
    if self.size == size {
      log::warn!("Ignoring resize event with same size");
      return
    }
    log::debug!("resizing surface to {:?}", size);
    self.size = size;
    self.surface_config.width = size.width;
    self.surface_config.height = size.height;
    self.surface.configure(&self.device, &self.surface_config);
  }

  pub fn reconfigure(&self) {
    self.surface.configure(&self.device, &self.surface_config);
  }

  //getters:
  pub fn size(&self) -> PhysicalSize<u32> {
    self.size
  }

  pub fn window(&self) -> &Window {
    &self.window
  }

  pub fn surface(&self) -> &wgpu::Surface<'static> {
    &self.surface
  }

  pub fn device(&self) -> &wgpu::Device {
    &self.device
  }

  pub fn queue(&self) -> &wgpu::Queue {
    &self.queue
  }

  pub fn surface_config(&self) -> &wgpu::SurfaceConfiguration {
    &self.surface_config
  }
}

pub fn render_master(storages: AllStoragesViewMut) {
  let renderer = storages.borrow::<UniqueView<Renderer>>().unwrap();

  let mut encoder = renderer.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
    label: Some("main_encoder"),
  });
  let surface_texture = renderer.surface().get_current_texture().unwrap();
  let surface_view = surface_texture.texture.create_view(&wgpu::TextureViewDescriptor::default());

  {
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

    // render_pass.set_pipeline(&renderer.pipeline);
    // render_pass.set_bind_group(0, &renderer.bind_group, &[]);
    // render_pass.set_vertex_buffer(0, renderer.vertex_buffer.slice(..));
    // render_pass.set_index_buffer(renderer.index_buffer.slice(..));
    // render_pass.draw_indexed(0..renderer.num_indices, 0, 0..1);
  }

  renderer.queue().submit(Some(encoder.finish()));
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
