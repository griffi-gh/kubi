use std::sync::Arc;
use fontdue::Metrics;
use glam::{IVec2, UVec2, uvec2, ivec2};
use hashbrown::HashMap;
use rect_packer::DensePacker;

use crate::IfModified;

use super::font::{FontHandle, FontManager};

#[derive(PartialEq, Eq, Hash)]
struct GlyphCacheKey {
  font_index: usize,
  character: char,
  size: u8,
}

pub struct GlyphCacheEntry {
  pub data: Vec<u8>,
  pub metrics: Metrics,
  pub position: IVec2,
  pub size: UVec2,
}

#[derive(Clone, Copy, Debug)]
pub struct FontTextureInfo<'a> {
  pub modified: bool,
  pub data: &'a [u8],
  pub size: UVec2,
}

impl<'a> IfModified<FontTextureInfo<'a>> for FontTextureInfo<'a> {
  fn if_modified(&self) -> Option<&Self> {
    match self.modified {
      true => Some(self),
      false => None,
    }
  }
}

// impl<'a> FontTextureInfo<'a> {
//   fn if_modified(&self) -> Option<Self> {
//     match self.modified {
//       true => Some(*self),
//       false => None,
//     }
//   }
// }

pub struct FontTextureManager {
  glyph_cache: HashMap<GlyphCacheKey, Arc<GlyphCacheEntry>>,
  packer: DensePacker,
  font_texture: Vec<u8>,
  font_texture_size: UVec2,
  modified: bool,
}

impl FontTextureManager {
  pub fn new(size: UVec2) -> Self {
    FontTextureManager {
      glyph_cache: HashMap::new(),
      packer: DensePacker::new(size.x as i32, size.y as i32),
      font_texture: vec![0; (size.x * size.y * 4) as usize],
      font_texture_size: size,
      modified: false,
    }
  }

  pub fn reset_modified(&mut self) {
    self.modified = false;
  }

  pub fn info(&self) -> FontTextureInfo {
    FontTextureInfo {
      modified: self.modified,
      data: &self.font_texture,
      size: self.font_texture_size,
    }
  }

  /// Either looks up the glyph in the cache or renders it and adds it to the cache.
  pub fn glyph(&mut self, font_manager: &FontManager, font_handle: FontHandle, character: char, size: u8) -> Arc<GlyphCacheEntry> {
    let key = GlyphCacheKey {
      font_index: font_handle.0,
      character,
      size,
    };
    if let Some(entry) = self.glyph_cache.get(&key) {
      return Arc::clone(entry);
    }
    let font = font_manager.get(font_handle).unwrap();
    let (metrics, bitmap) = font.rasterize(character, size as f32);
    log::debug!("rasterized glyph: {}, {:?}, {:?}", character, metrics, bitmap);
    let texture_position = self.packer.pack(metrics.width as i32, metrics.height as i32, false).unwrap();
    let texture_size = uvec2(metrics.width as u32, metrics.height as u32);
    let entry = Arc::new(GlyphCacheEntry {
      data: bitmap,
      metrics,
      position: ivec2(texture_position.x, texture_position.y),
      size: texture_size,
    });
    self.glyph_cache.insert_unique_unchecked(key, Arc::clone(&entry));
    self.glyph_place(&entry);
    self.modified = true;
    entry
  }

  /// Place glyph onto the font texture.
  fn glyph_place(&mut self, entry: &GlyphCacheEntry) {
    let tex_size = self.font_texture_size;
    let GlyphCacheEntry { size, position, data, .. } = entry;
    //println!("{size:?} {position:?}");
    for y in 0..size.y {
      for x in 0..size.x {
        let src = (size.x * y + x) as usize;
        let dst = (tex_size.x * (y + position.y as u32) + (x + position.x as u32)) as usize * 4;
        self.font_texture[dst..=(dst + 3)].copy_from_slice(&[255, 255, 255, data[src]]);
        //print!("{} ", if data[src] > 0 {'#'} else {'.'});
        //print!("{src} {dst} / ");
      }
      //println!();
    }
  }

  // pub fn glyph(&mut self, font_manager: &FontManager, font_handle: FontHandle, character: char, size: u8) -> Arc<GlyphCacheEntry> {
  //   let (is_new, glyph) = self.glyph_allocate(font_manager, font_handle, character, size);
  //   if is_new {
  //     self.glyph_place(&glyph);
  //     self.modified = true;
  //   }
  //   glyph
  // }
}

impl Default for FontTextureManager {
  fn default() -> Self {
    Self::new(uvec2(1024, 1024))
  }
}
