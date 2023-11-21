use std::num::NonZeroU32;
use raw_window_handle::HasRawWindowHandle;
use shipyard::{Unique, NonSendSync, UniqueView, UniqueViewMut, View, IntoIter, AllStoragesView};
use winit::{
  event_loop::EventLoopWindowTarget,
  window::{WindowBuilder, Fullscreen, Window},
  dpi::PhysicalSize
};
use glium::{Display, Surface, Version, Api};
use glutin::{surface::WindowSurface, display::{GetGlDisplay, GlDisplay}, context::NotCurrentGlContext};
use glam::{Vec3, UVec2};
use crate::{events::WindowResizedEvent, settings::{GameSettings, FullscreenMode}};

pub mod primitives;
pub mod world;
pub mod selection_box;
pub mod entities;

#[derive(Unique)]
#[repr(transparent)]
pub struct RenderTarget(pub glium::Frame);

#[derive(Unique)]
#[repr(transparent)]
pub struct BackgroundColor(pub Vec3);

#[derive(Unique, Clone, Copy)]
#[repr(transparent)]
pub struct WindowSize(pub UVec2);

#[derive(Unique)]
pub struct Renderer {
  pub window: Window,
  pub display: Display<WindowSurface>,
}

impl Renderer {
  pub fn init(event_loop: &EventLoopWindowTarget<()>, settings: &GameSettings) -> Self {
    log::info!("initializing display");

    let wb = WindowBuilder::new()
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

    // First we start by opening a new Window
    let display_builder = glutin_winit::DisplayBuilder::new().with_window_builder(Some(wb));
    let config_template_builder = glutin::config::ConfigTemplateBuilder::new();
    let (window, gl_config) = display_builder
      .build(&event_loop, config_template_builder, |mut configs| {
        configs.next().unwrap()
      })
      .unwrap();
    let window = window.unwrap();

    // Now we get the window size to use as the initial size of the Surface
    let (width, height): (u32, u32) = window.inner_size().into();
    let attrs = glutin::surface::SurfaceAttributesBuilder::<glutin::surface::WindowSurface>::new().build(
      window.raw_window_handle(),
      NonZeroU32::new(width).unwrap(),
      NonZeroU32::new(height).unwrap(),
    );

    // Finally we can create a Surface, use it to make a PossiblyCurrentContext and create the glium Display
    let surface = unsafe { gl_config.display().create_window_surface(&gl_config, &attrs).unwrap() };
    let context_attributes = glutin::context::ContextAttributesBuilder::new().build(Some(window.raw_window_handle()));
    let current_context = unsafe {
      gl_config.display().create_context(&gl_config, &context_attributes).expect("failed to create context")
    }.make_current(&surface).unwrap();
    let display = Display::from_context_surface(current_context, surface).unwrap();

    //TODO MIGRATION
    // let cb = ContextBuilder::new()
    //   //.with_srgb(false)
    //   .with_depth_buffer(24)
    //   .with_multisampling(settings.msaa.unwrap_or_default())
    //   .with_vsync(settings.vsync)
    //   .with_gl_profile(GlProfile::Core)
    //   .with_gl(GlRequest::Latest);

    // let display = Display::new(wb, cb)
    //   .expect("Failed to create a glium Display");

    log::info!("Vendor: {}", display.get_opengl_vendor_string());
    log::info!("Renderer: {}", display.get_opengl_renderer_string());
    log::info!("OpenGL: {}", display.get_opengl_version_string());
    log::info!("Supports GLSL: {:?}", display.get_supported_glsl_version());
    log::info!("Framebuffer dimensions: {:?}", display.get_framebuffer_dimensions());
    if display.is_context_loss_possible() { log::warn!("OpenGL context loss possible") }
    if display.is_robust() { log::warn!("OpenGL implementation is not robust") }
    if display.is_debug() { log::info!("OpenGL context is in debug mode") }

    assert!(display.is_glsl_version_supported(&Version(Api::GlEs, 3, 0)), "GLSL ES 3.0 is not supported");

    Self { window, display }
  }
}

pub fn clear_background(
  mut target: NonSendSync<UniqueViewMut<RenderTarget>>,
  color: UniqueView<BackgroundColor>,
) {
  target.0.clear_color_srgb_and_depth((color.0.x, color.0.y, color.0.z, 1.), 1.);
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
