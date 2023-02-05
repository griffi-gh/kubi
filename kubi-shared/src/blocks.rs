use bincode::{Encode, Decode};
use strum::EnumIter;

#[derive(Encode, Decode, Clone, Copy, Debug, PartialEq, Eq, EnumIter)]
#[repr(u8)]
pub enum Block {
  Air,
  Stone,
  Dirt,
  Grass,
  Sand,
  Cobblestone,
}
