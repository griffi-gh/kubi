use shipyard::{AllStoragesView, Unique, NonSendSync, UniqueView, UniqueViewMut};
use crate::rendering::Renderer;
use winit::window::CursorGrabMode;

#[derive(Unique)]
pub struct CursorLock(pub bool);

pub fn update_cursor_lock_state(
  lock: UniqueView<CursorLock>,
  renderer: NonSendSync<UniqueView<Renderer>>
) {
  #[cfg(not(target_os = "android"))]
  if lock.is_inserted_or_modified() {
    let window = &renderer.window;
    if lock.0 {
      window.set_cursor_grab(CursorGrabMode::Confined)
        .or_else(|_| window.set_cursor_grab(CursorGrabMode::Locked))
        .expect("Failed to lock the cursor");
    } else {
      window.set_cursor_grab(CursorGrabMode::None)
        .expect("Failed to unlock the cursor");
    }
    renderer.window.set_cursor_visible(!lock.0);
  }
}

pub fn insert_lock_state(
  storages: AllStoragesView
) {
  storages.add_unique(CursorLock(false))
}

pub fn lock_cursor_now(
  mut lock: UniqueViewMut<CursorLock>
) {
  lock.0 = true
}
