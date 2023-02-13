use bincode::{Encode, Decode};
use crate::common::ClientId;

pub trait Message: Encode + Decode + Clone {}
impl<T: Encode + Decode + Clone> Message for T {}

#[repr(u8)]
#[derive(Encode, Decode)]
pub enum ClientPacket<T> where T: Message {
  Connect {
    inner_protocol: u16,
    user_protocol: u16,
  }, //should always stay 0!
  Data(T),
  Disconnect,
  Heartbeat,
}

#[derive(Encode, Decode)]
pub struct IdClientPacket<T: Message>(pub Option<ClientId>, pub ClientPacket<T>);

#[repr(u8)]
#[derive(Encode, Decode)]
pub enum ServerPacket<T> where T: Message {
  ProtoDisconnect = 0,
  Data(T),
  Disconnected(String),
  Connected(ClientId),
  Heartbeat,
}

#[derive(Encode, Decode)]
pub struct IdServerPacket<T: Message>(pub Option<ClientId>, pub ServerPacket<T>);
