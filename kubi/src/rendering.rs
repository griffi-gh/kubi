use shipyard::{Unique, Workload, IntoWorkload, WorkloadModificator, AllStoragesView};
use winit::{
  event_loop::EventLoop,
  window::{Window, WindowBuilder, Fullscreen},
  dpi::PhysicalSize,
};
use glam::{Vec3, Mat4};
use pollster::FutureExt as _;
use crate::settings::{GameSettings, FullscreenMode};

use self::{primitives::init_primitives, shaders::compile_shaders, camera_uniform::init_camera_uniform};

pub mod shaders;
pub mod primitives;
pub mod selection_box;
pub mod entities;
pub mod world;
pub mod camera_uniform;

//TODO remove this if possible
pub const OPENGL_TO_WGPU_MATRIX: Mat4 = Mat4::from_cols_array(&[
  1.0, 0.0, 0.0, 0.0,
  0.0, 1.0, 0.0, 0.0,
  0.0, 0.0, 0.5, 0.0,
  0.0, 0.0, 0.5, 1.0,
]);

pub struct RenderData {
  pub output: wgpu::SurfaceTexture,
  pub view: wgpu::TextureView,
  pub encoder: wgpu::CommandEncoder,
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

    Self {
      window, instance, surface, adapter, device, queue, size, config,
    }
  }

  /// do not call from async functions
  pub fn init_blocking(event_loop: &EventLoop<()>, settings: &GameSettings) -> Self {
    Self::init(event_loop, settings).block_on()
  }

  /// Start a new frame
  pub fn begin(&self) -> RenderData {
    //Surface texture
    let output = self.surface.get_current_texture().unwrap();

    //View
    let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

    //Encoder
    let encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
      label: Some("RenderEncoder"),
    });

    RenderData { output, view, encoder }
  }

  pub fn end(&self, target: RenderData) {
    self.queue.submit([target.encoder.finish()]);
    target.output.present();
  }
  
  /// Resize the surface
  /// ## Panics:
  /// - ...if any dimension is equal to zero
  pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
    //XXX: just check instead?
    assert!(new_size.width > 0, "width cannot be zero");
    assert!(new_size.height > 0, "height cannot be zero");
    self.size = new_size;
    self.config.width = new_size.width;
    self.config.height = new_size.height;
    self.surface.configure(&self.device, &self.config);
  }
}

/// THIS DOES NOT INIT [`Renderer`]!
pub fn init_rendering_internals() -> Workload {
  (
    init_camera_uniform,
    init_primitives,
    compile_shaders,
    (
      world::init_gpu_data,
    ).into_workload().after_all(compile_shaders),
  ).into_workload()
}

macro_rules! vertex_attributes {
  (
    $T: ident,
    $(
      $name:ident: $vertex_format:ident
    ),*
  ) => {
    impl $T {
      pub const VERTEX_ATTRIBUTES: [::wgpu::VertexAttribute; [$(stringify!($name)),*].len()] = {
        let mut offset = 0;
        let mut shader_location = 0;
        [
          $(
            {
              let attribute = wgpu::VertexAttribute {
                format: ::wgpu::VertexFormat::$vertex_format,
                offset, shader_location,
              };
              #[allow(unused_assignments)] {
                shader_location += 1;
                offset += ::wgpu::VertexFormat::$vertex_format.size();
              }
              attribute
            },
          )*
        ]
      };
    }
  };
}
pub(crate) use vertex_attributes;
