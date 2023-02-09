use glam::{Vec4, vec4};

#[inline(always)]
pub fn color_rgba(r: u8, g: u8, b: u8, a: u8) -> Vec4 {
  vec4(r as f32 / 255., g as f32 / 255., b as f32 / 255., a as f32 / 255.)
}

#[inline(always)]
pub const fn color_hex(c: u32) -> Vec4 {
  let c = c.to_le_bytes();
  vec4(c[0] as f32, c[1] as f32, c[2] as f32, c[3] as f32)
}
