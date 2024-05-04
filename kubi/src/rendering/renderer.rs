use pollster::FutureExt;
use shipyard::Unique;
use winit::{
  event_loop::ActiveEventLoop,
  window::{Fullscreen, Window},
  dpi::PhysicalSize
};
use crate::settings::{GameSettings, FullscreenMode};

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
      backends: wgpu::Backends::BROWSER_WEBGPU | wgpu::Backends::VULKAN | wgpu::Backends::GL,
      ..Default::default()
    });

    // Create a surface with `create_surface_unsafe` to get a surface with 'static lifetime
    // It should never outlive the window it's created from
    let surface = unsafe {
      let target = wgpu::SurfaceTargetUnsafe::from_window(&window).unwrap();
      instance.create_surface_unsafe(target).unwrap()
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
