#[derive(Clone, Debug)]
pub struct GameOptions {
  pub render_distance: u8,
}
impl Default for GameOptions {
  fn default() -> Self {
    Self {
      render_distance: 8,
    }
  }
}
