#[derive(Clone, Debug)]
pub struct GameOptions {
  pub render_distance: u8,
  pub debug_wireframe_mode: bool,
}
impl Default for GameOptions {
  fn default() -> Self {
    Self {
      render_distance: if cfg!(debug_assertions) { 8 } else { 16 },
      debug_wireframe_mode: false,
    }
  }
}
