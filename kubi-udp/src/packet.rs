use bincode::{Encode, Decode};

#[repr(u8)]
#[derive(Encode, Decode)]
pub enum ClientPacket<T> where T: Encode + Decode {
  Data(T),
  Connect,
  Disconnect,
  Heartbeat,
}

#[repr(u8)]
#[derive(Encode, Decode)]
pub enum ServerPacket<T> where T: Encode + Decode {
  Data(T),
  Connected,
  Disconnected,
}
