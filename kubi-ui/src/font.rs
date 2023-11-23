use fontdue::Font;

#[cfg(feature = "builtin_font")]
const BIN_FONT: &[u8] = include_bytes!("../assets/font/ProggyTiny.ttf");

pub struct FontRenderer {
  pub font_stack: Vec<Font>,
  pub cache: ()
}
