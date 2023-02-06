use std::num::NonZeroU8;
use bincode::{Encode, Decode};

pub type ClientId = NonZeroU8;
pub const MAX_CLIENTS: usize = u8::MAX as _;

#[derive(Default, Encode, Decode, Clone)]
#[repr(u8)]
pub enum DisconnectReason {
  #[default]
  NotConnected,
  ClientDisconnected,
  KickedByServer(String),
  ClientTimeout,
  ServerTimeout,
}
