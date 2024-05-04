use shipyard::{Unique, NonSendSync, UniqueView, UniqueViewMut, View, IntoIter, AllStoragesView};
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

#[derive(Unique)]
#[repr(transparent)]
pub struct BackgroundColor(pub Vec3);

#[derive(Unique, Clone, Copy)]
#[repr(transparent)]
pub struct WindowSize(pub UVec2);

#[derive(Unique)]
pub struct Renderer {
  pub window: Window,
}

impl Renderer {
  pub fn init(event_loop: &ActiveEventLoop, settings: &GameSettings) -> Self {
    log::info!("initializing display");

    let wb = WindowAttributes::new()
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

    Self { window }
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
