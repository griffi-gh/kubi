use shipyard::{AllStoragesView, Unique, NonSendSync, UniqueView};
use crate::rendering::Renderer;
use glium::glutin::window::CursorGrabMode;

#[derive(Unique)]
#[track(All)]
pub struct CursorLock(pub bool);

pub fn update_cursor_lock_state(
  lock: UniqueView<CursorLock>,
  display: NonSendSync<UniqueView<Renderer>>
) {
  if lock.is_inserted_or_modified() {
    let gl_window = display.display.gl_window();
    let window = gl_window.window();
    window.set_cursor_grab(match lock.0 {
      true  => CursorGrabMode::Confined,
      false => CursorGrabMode::None,
    }).expect("Failed to change cursor grab state");
    window.set_cursor_visible(!lock.0);
  }
}

pub fn insert_lock_state(
  storages: AllStoragesView
) {
  storages.add_unique(CursorLock(false))
}
