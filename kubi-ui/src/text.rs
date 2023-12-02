use std::sync::Arc;

mod font;
mod ftm;

use font::FontManager;
pub use font::FontHandle;
use fontdue::{Font, FontSettings};
use ftm::FontTextureManager;
pub use ftm::{FontTextureInfo, GlyphCacheEntry};

pub struct TextRenderer {
  fm: FontManager,
  ftm: FontTextureManager,
}

impl TextRenderer {
  pub fn new() -> Self {
    Self {
      fm: FontManager::new(),
      ftm: FontTextureManager::default(),
    }
  }

  pub fn add_font_from_bytes(&mut self, font: &[u8]) -> FontHandle {
    self.fm.add_font(Font::from_bytes(font, FontSettings::default()).unwrap())
  }

  pub fn reset_frame(&mut self) {
    self.ftm.reset_modified();
  }

  pub fn font_texture(&self) -> FontTextureInfo {
    self.ftm.info()
  }

  pub fn glyph(&mut self, font_handle: FontHandle, character: char, size: u8) -> Arc<GlyphCacheEntry> {
    self.ftm.glyph(&self.fm, font_handle, character, size)
  }
}

impl Default for TextRenderer {
  fn default() -> Self {
    Self::new()
  }
}
