use shipyard::{Unique, NonSendSync, UniqueView, UniqueViewMut, View, IntoIter, AllStoragesView};
use winit::{
  event_loop::EventLoop,
  window::{Window, WindowBuilder, Fullscreen},
  dpi::PhysicalSize,
};
use glam::{Vec3, UVec2};
use pollster::FutureExt as _;
use crate::{events::WindowResizedEvent, settings::{GameSettings, FullscreenMode}};

pub mod primitives;
pub mod world;
pub mod selection_box;
pub mod entities;

#[derive(Unique)]
#[repr(transparent)]
pub struct RenderTarget(pub ());
impl Drop for RenderTarget {

}

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
}
impl Renderer {
  pub async fn init(event_loop: &EventLoop<()>, settings: &GameSettings) -> Self {
    log::info!("initializing display");
    
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

    //Create wgpu stuff
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
      backends: wgpu::Backends::all(),
      dx12_shader_compiler: if cfg!(all(windows, feature = "dx12-dxc")) {
        wgpu::Dx12Compiler::Dxc { dxil_path: None, dxc_path: None }
      } else {
        wgpu::Dx12Compiler::Fxc
      }
    });

    let surface = unsafe {
      instance.create_surface(&window)
    }.expect("Failed to create a Surface");

    let adapter = instance.request_adapter(
      &wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
      },
    ).await.expect("Failed to create wgpu adapter");
  
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
    
    log::info!("Adapter: {:?}", adapter.get_info());

    Self { window, instance, surface, adapter, device, queue, size: PhysicalSize::default() }
  }

  /// do not call from async functions
  pub fn init_blocking(event_loop: &EventLoop<()>, settings: &GameSettings) -> Self {
    Self::init(event_loop, settings).block_on()
  }

  /// Start a new frame
  pub fn render() -> RenderTarget {

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
