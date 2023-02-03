use std::num::NonZeroU8;
use bincode::{Encode, Decode};

pub type ClientId = NonZeroU8;

#[derive(Default, Encode, Decode)]
#[repr(u8)]
pub enum DisconnectReason {
  #[default]
  NotConnected,
  ClientDisconnected,
  KickedByServer,
  ClientTimeout,
  ServerTimeout,
}
