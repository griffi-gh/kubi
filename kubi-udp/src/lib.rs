pub mod client;
pub mod server;
pub(crate) mod packet;
pub(crate) mod common;
pub use common::{ClientId, DisconnectReason};

//pub(crate) trait Serializable: bincode::Encode + bincode::Decode {}
pub(crate) const BINCODE_CONFIG: bincode::config::Configuration<bincode::config::LittleEndian, bincode::config::Varint, bincode::config::SkipFixedArrayLength> = bincode::config::standard()
  .with_little_endian()
  .with_variable_int_encoding()
  .skip_fixed_array_length();
