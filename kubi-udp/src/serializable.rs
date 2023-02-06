use anyhow::Result;
use crate::BINCODE_CONFIG;

pub trait Serializable: bincode::Encode + bincode::Decode {
  fn serialize(&self, buf: &mut [u8]) -> Result<()>;
  fn deserialize(buf: &[u8]) -> Result<Self>;

  fn serialize_to_vec(&self) -> Result<Vec<u8>> {
    let mut buf = Vec::new();
    self.serialize(&mut buf)?;
    Ok(buf)
  }
}
impl<T: bincode::Encode + bincode::Decode> Serializable for T {
  fn serialize(&self, buf: &mut [u8]) -> Result<()> {
    bincode::encode_into_slice(self, buf, BINCODE_CONFIG)?;
    Ok(())
  }
  fn deserialize(buf: &[u8]) -> Result<Self> {
    bincode::decode_from_slice(buf, BINCODE_CONFIG)?.0
  }
}
