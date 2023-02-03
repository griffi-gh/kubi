use bincode::{Encode, Decode};
use crate::common::ClientId;

#[repr(u8)]
#[derive(Encode, Decode)]
pub enum ClientPacket<T> where T: Encode + Decode {
  Data(T),
  Connect,
  Disconnect,
  Heartbeat,
}

#[derive(Encode, Decode)]
pub struct IdClientPacket<T: Encode + Decode>(pub Option<ClientId>, pub ClientPacket<T>);

#[repr(u8)]
#[derive(Encode, Decode)]
pub enum ServerPacket<T> where T: Encode + Decode {
  Data(T),
  Connected,
  Disconnected,
}
