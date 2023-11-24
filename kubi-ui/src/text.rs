use std::sync::Arc;
use fontdue::{Font, Metrics};
use glam::{IVec2, UVec2, uvec2, ivec2};
use hashbrown::HashMap;
use rect_packer::DensePacker;

#[cfg(feature = "builtin_font")]
const BIN_FONT: &[u8] = include_bytes!("../assets/font/ProggyTiny.ttf");

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct FontHandle(pub(crate) usize);

#[derive(PartialEq, Eq, Hash)]
struct GlyphCacheKey {
  font_index: usize,
  character: char,
  size: u8,
}

struct GlyphCacheEntry {
  pub data: Vec<u8>,
  pub metrics: Metrics,
  pub texture_position: IVec2,
  pub texture_size: UVec2,
}

pub struct TextRenderer {
  fonts: Vec<Font>,
  glyph_cache: HashMap<GlyphCacheKey, Arc<GlyphCacheEntry>>,
  packer: DensePacker,
  font_texture: Vec<u8>,
  font_texture_size: UVec2,
}

impl TextRenderer {
  pub fn new(size: UVec2) -> Self {
    let mut renderer = TextRenderer {
      fonts: Vec::new(),
      glyph_cache: HashMap::new(),
      packer: DensePacker::new(size.x as i32, size.y as i32),
      font_texture: vec![0; (size.x * size.y) as usize],
      font_texture_size: size,
    };
    #[cfg(feature = "builtin_font")]
    {
      let font = Font::from_bytes(BIN_FONT, fontdue::FontSettings::default()).unwrap();
      renderer.add_font(font);
    }
    renderer
  }

  /// Add a (fontdue) font to the renderer.
  pub fn add_font(&mut self, font: Font) -> FontHandle {
    self.fonts.push(font);
    FontHandle(self.fonts.len() - 1)
  }

  /// Either looks up the glyph in the cache or renders it and adds it to the cache.
  pub fn glyph(&mut self, font: FontHandle, character: char, size: u8) -> Arc<GlyphCacheEntry> {
    let key = GlyphCacheKey {
      font_index: font.0,
      character,
      size,
    };
    if let Some(entry) = self.glyph_cache.get(&key) {
      return Arc::clone(entry);
    }
    let font = &self.fonts[key.font_index];
    let (metrics, bitmap) = font.rasterize(character, size as f32);
    let texture_position = self.packer.pack(metrics.width as i32, metrics.height as i32, false).unwrap();
    let texture_size = uvec2(metrics.width as u32, metrics.height as u32);
    let entry = Arc::new(GlyphCacheEntry {
      data: bitmap,
      metrics,
      texture_position: ivec2(texture_position.x, texture_position.y),
      texture_size,
    });
    self.glyph_cache.insert_unique_unchecked(key, Arc::clone(&entry));
    entry
  }
}

impl Default for TextRenderer {
  fn default() -> Self {
    Self::new(uvec2(2048, 2048))
  }
}
