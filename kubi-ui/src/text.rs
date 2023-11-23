use fontdue::{Font, Metrics};
use hashbrown::HashMap;
use nohash_hasher::BuildNoHashHasher;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

#[cfg(feature = "builtin_font")]
const BIN_FONT: &[u8] = include_bytes!("../assets/font/ProggyTiny.ttf");

// pub struct Glyph {
//   pub metrics: Metrics,
//   pub data: Box<[u8]>
// }

// pub struct FontData {
//   font: Font,
//   cache: HashMap<(u8, char), Glyph, BuildNoHashHasher<u8>>
// }

// impl FontData {
//   pub fn new() -> Self {
//     FontData {
//       font: Font::from_bytes(BIN_FONT, fontdue::FontSettings::default()).unwrap(),
//       cache: HashMap::default()
//     }
//   }

//   fn prebake(&mut self, size: u8) {
//     self.cache = (33..=126).par_bridge().map(|c| {
//       let (metrics, data) = self.font.rasterize(
//         char::from(c),
//         c as f32,
//       );
//       Glyph { metrics, data: data.into_boxed_slice() }
//     }).collect();
//   }
// }

pub struct TextRenderer {
  font_stack: Vec<Font>,
  cache: ()
}

impl TextRenderer {
  pub fn new() -> Self {
    let font = Font::from_bytes(BIN_FONT, fontdue::FontSettings::default()).unwrap();
    TextRenderer {
      font_stack: vec![font],
      cache: ()
    }
  }
}

impl Default for TextRenderer {
  fn default() -> Self {
    Self::new()
  }
}
