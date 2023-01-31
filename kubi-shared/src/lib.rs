pub mod blocks;
pub mod networking;
pub mod worldgen;
pub mod chunk;

pub(crate) const BINCODE_CONFIG: bincode::config::Configuration<bincode::config::LittleEndian, bincode::config::Varint, bincode::config::SkipFixedArrayLength> = bincode::config::standard()
  .with_little_endian()
  .with_variable_int_encoding()
  .skip_fixed_array_length();
