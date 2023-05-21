use shipyard::{Unique, AllStoragesView};

pub enum FullscreenMode {
  Borderless,
  Exclusive,
}

pub struct FullscreenSettings {
  pub mode: FullscreenMode,
}

#[derive(Unique)]
pub struct GameSettings {
  pub vsync: bool,
  pub fullscreen: Option<FullscreenSettings>,
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
      vsync: false,
      fullscreen: None,
      msaa: Some(4), //not used yet
      max_anisotropy: Some(16),
      render_distance: 6,
      mouse_sensitivity: 1.,
      debug_draw_current_chunk_border: cfg!(not(target_os = "android")) && cfg!(debug_assertions),
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
