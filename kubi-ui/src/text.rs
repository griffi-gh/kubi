use std::sync::Arc;

mod font;
mod ftm;

use font::FontManager;
pub use font::FontHandle;
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
