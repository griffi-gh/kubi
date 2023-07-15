use shipyard::{Unique, NonSendSync, UniqueView, UniqueViewMut, View, IntoIter, AllStoragesView};
use wgpu::SurfaceConfiguration;
use winit::{
  event_loop::EventLoop,
  window::{Window, WindowBuilder, Fullscreen},
  dpi::PhysicalSize,
};
use glam::{Vec3, UVec2};
use pollster::FutureExt as _;
use crate::{events::WindowResizedEvent, settings::{GameSettings, FullscreenMode}};

pub mod shaders;
pub mod primitives;
pub mod world;
pub mod selection_box;
pub mod entities;

#[derive(Unique)]
#[repr(transparent)]
pub struct RenderTarget(pub ());

#[derive(Unique)]
#[repr(transparent)]
pub struct BackgroundColor(pub Vec3);

#[derive(Unique)]
pub struct Renderer {
  pub window: Window,
  pub instance: wgpu::Instance,
  pub surface: wgpu::Surface,
  pub adapter: wgpu::Adapter,
  pub device: wgpu::Device,
  pub queue: wgpu::Queue,
  pub size: PhysicalSize<u32>,
  pub config: wgpu::SurfaceConfiguration,
}
impl Renderer {
  pub async fn init(event_loop: &EventLoop<()>, settings: &GameSettings) -> Self {
    log::info!("initializing display");
    
    // ========== Create a winit window ==========

    //Build window
    let window = WindowBuilder::new()
      .with_title("kubi")
      .with_maximized(true)
      .with_min_inner_size(PhysicalSize::new(640, 480))
      .build(event_loop)
      .expect("Window creation failed");

    //Enable fullscreen (on supported platforms; if enabled in settings)
    #[cfg(not(target_os = "android"))]
    window.set_fullscreen(settings.fullscreen.as_ref().and_then(|fullscreen| {
      match fullscreen.mode {
        FullscreenMode::Exclusive => {
          Some(Fullscreen::Borderless(window.current_monitor()))
        }
        FullscreenMode::Borderless => {
          window.current_monitor().and_then(|monitor| {
            monitor.video_modes().next().map(|video_mode| {
              Fullscreen::Exclusive(video_mode)
            })
          })
        }
      }
    }));
    
    let size = window.inner_size();

    // ========== Create wgpu stuff ==========
    
    // instance

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
      backends: wgpu::Backends::all(),
      dx12_shader_compiler: if cfg!(all(windows, feature = "dx12-dxc")) {
        // Better, but requires shipping ms dxil dlls
        wgpu::Dx12Compiler::Dxc { dxil_path: None, dxc_path: None }
      } else {
        wgpu::Dx12Compiler::Fxc
      }
    });

    // surface

    let surface = unsafe {
      instance.create_surface(&window)
    }.expect("Failed to create a Surface");

    // adapter

    let adapter = instance.request_adapter(
      &wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
      },
    ).await.expect("Failed to create wgpu adapter");
    log::info!("Adapter: {:?}", adapter.get_info());
    
    // device/queue

    let (device, queue) = adapter.request_device(
      &wgpu::DeviceDescriptor {
        features: wgpu::Features::empty(),
        limits: if cfg!(target_arch = "wasm32") {
          wgpu::Limits::downlevel_webgl2_defaults()
        } else if cfg!(target_arch = "android") {
          wgpu::Limits::downlevel_defaults()
        } else {
          wgpu::Limits::default()
        },
        label: None,
      },
      None,
    ).await.unwrap();
    
    // surf. format

    let surface_capabilities = surface.get_capabilities(&adapter);
    let surface_format = surface_capabilities.formats.iter()
      .copied()
      .find(|f| f.is_srgb())
      .unwrap_or(surface_capabilities.formats[0]);

    // config

    let config = wgpu::SurfaceConfiguration {
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
      format: surface_format,
      width: size.width,
      height: size.height,
      present_mode: match settings.vsync {
        true  => wgpu::PresentMode::AutoVsync,
        false => wgpu::PresentMode::AutoNoVsync,
      },
      alpha_mode: surface_capabilities.alpha_modes[0],
      view_formats: vec![],
    };
    surface.configure(&device, &config);

    Self { window, instance, surface, adapter, device, queue, size, config }
  }

  /// do not call from async functions
  pub fn init_blocking(event_loop: &EventLoop<()>, settings: &GameSettings) -> Self {
    Self::init(event_loop, settings).block_on()
  }

  /// Start a new frame
  pub fn render() -> RenderTarget {
    todo!()
  }

  /// Resize the surface
  /// ## Panics:
  /// - ...if any dimension is equal to zero
  pub fn resize(&self, new_size: PhysicalSize<u32>) {
    //XXX: just check instead?
    assert!(new_size.width > 0, "width cannot be zero");
    assert!(new_size.height > 0, "height cannot be zero");
    self.size = new_size;
    self.config.width = new_size.width;
    self.config.height = new_size.height;
    self.surface.configure(&self.device, &self.config);
  }
}

pub fn clear_background(
  mut target: NonSendSync<UniqueViewMut<RenderTarget>>,
  color: UniqueView<BackgroundColor>,
) {

}

#[deprecated]
pub fn if_resized (
  resize: View<WindowResizedEvent>,
) -> bool {
  resize.len() > 0
}
