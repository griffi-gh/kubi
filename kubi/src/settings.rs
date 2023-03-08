use shipyard::{Unique, AllStoragesView};

#[derive(Unique)]
pub struct GameSettings {
  /// there's a 1 chunk border of loaded but invisible around this
  pub render_distance: u8,
  pub mouse_sensitivity: f32,
  pub debug_draw_current_chunk_border: bool,
}
impl Default for GameSettings {
  fn default() -> Self {
    Self {
      render_distance: 6,
      mouse_sensitivity: 1.,
      debug_draw_current_chunk_border: cfg!(debug_assertions),
    }
  }
}

pub fn load_settings(
  storages: AllStoragesView
) {
  //todo
  storages.add_unique(GameSettings::default());
}
