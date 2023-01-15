pub mod textures;

use textures::Textures;

pub struct Assets {
  pub textures: Textures
}
impl Assets {
  /// Load all assets synchronously
  pub fn load_all_sync(display: &glium::Display) -> Self {
    Self {
      textures: Textures::load_sync(display)
    }
  }
}
