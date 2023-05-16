use shipyard::{Unique, AllStoragesView};

#[derive(Unique)]
pub struct GameSettings {
  pub msaa: Option<u16>,
  pub max_anisotropy: Option<u16>,
  /// there's a 1 chunk border of loaded but invisible around this
  pub render_distance: u8,
  pub mouse_sensitivity: f32,
  pub debug_draw_current_chunk_border: bool,
}
impl Default for GameSettings {
  fn default() -> Self {
    Self {
      msaa: Some(4), //not used yet
      max_anisotropy: Some(16),
      render_distance: 6,
      mouse_sensitivity: 1.,
      debug_draw_current_chunk_border: cfg!(debug_assertions),
    }
  }
}

pub fn load_settings(
  storages: AllStoragesView
) {
  log::info!("loading game settings");
  //todo
  storages.add_unique(GameSettings::default());
}
