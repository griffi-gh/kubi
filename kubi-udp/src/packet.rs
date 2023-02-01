use bincode::{Encode, Decode};

#[repr(u8)]
#[derive(Encode, Decode)]
pub enum ClientPacket<T> where T: Encode + Decode {
  Connect,
  Disconnect,
  Heartbeat,
  Data(T)
}
