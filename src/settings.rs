use shipyard::Unique;

#[derive(Unique)]
pub struct GameSettings {
  //there's a 1 chunk border of loaded but invisible around this
  pub render_distance: u8,
}
impl Default for GameSettings {
  fn default() -> Self {
    Self {
      render_distance: 2
    }
  }
}
