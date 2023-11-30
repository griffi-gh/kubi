use std::sync::Arc;
use fontdue::{Font, Metrics};
use glam::{IVec2, UVec2, uvec2, ivec2};
use hashbrown::HashMap;
use rect_packer::DensePacker;

use super::font::{FontHandle, FontManager};



#[derive(PartialEq, Eq, Hash)]
struct GlyphCacheKey {
  font_index: usize,
  character: char,
  size: u8,
}

struct GlyphCacheEntry {
  pub data: Vec<u8>,
  pub metrics: Metrics,
  pub position: IVec2,
  pub size: UVec2,
}

pub struct FontTextureManager {
  glyph_cache: HashMap<GlyphCacheKey, Arc<GlyphCacheEntry>>,
  packer: DensePacker,
  font_texture: Vec<u8>,
  font_texture_size: UVec2,
  modified: bool,
}

impl FontTextureManager {
  pub fn new(size: UVec2) -> Self {
    let mut renderer = FontTextureManager {
      glyph_cache: HashMap::new(),
      packer: DensePacker::new(size.x as i32, size.y as i32),
      font_texture: vec![0; (size.x * size.y) as usize],
      font_texture_size: size,
      modified: false,
    };
    renderer
  }

  /// Either looks up the glyph in the cache or renders it and adds it to the cache.
  fn glyph_allocate(&mut self, font_manager: &FontManager, font_handle: FontHandle, character: char, size: u8) -> (bool, Arc<GlyphCacheEntry>) {
    let key = GlyphCacheKey {
      font_index: font_handle.0,
      character,
      size,
    };
    if let Some(entry) = self.glyph_cache.get(&key) {
      return (false, Arc::clone(entry));
    }
    let font = font_manager.get(font_handle).unwrap();
    let (metrics, bitmap) = font.rasterize(character, size as f32);
    let texture_position = self.packer.pack(metrics.width as i32, metrics.height as i32, false).unwrap();
    let texture_size = uvec2(metrics.width as u32, metrics.height as u32);
    let entry = Arc::new(GlyphCacheEntry {
      data: bitmap,
      metrics,
      position: ivec2(texture_position.x, texture_position.y),
      size: texture_size,
    });
    self.glyph_cache.insert_unique_unchecked(key, Arc::clone(&entry));
    (true, entry)
  }

  /// Place glyph onto the font texture.
  fn glyph_place(&mut self, entry: &GlyphCacheEntry) {
    let tex_size = self.font_texture_size;
    let GlyphCacheEntry { size, position, .. } = entry;
    for y in 0..size.y {
      for x in 0..size.x {
        let src = (size.x * y + x) as usize;
        let dst = (tex_size.x * (y + position.y as u32) + (x + position.x as u32)) as usize;
        self.font_texture[dst] = entry.data[src];
      }
    }
  }

  pub fn glyph(&mut self, font_manager: &FontManager, font_handle: FontHandle, character: char, size: u8) -> Arc<GlyphCacheEntry> {
    let (is_new, glyph) = self.glyph_allocate(font_manager, font_handle, character, size);
    if is_new {
      self.glyph_place(&glyph);
      self.modified = true;
    }
    glyph
  }
}

impl Default for FontTextureManager {
  fn default() -> Self {
    Self::new(uvec2(2048, 2048))
  }
}
