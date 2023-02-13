use std::num::NonZeroU8;

pub type ClientId = NonZeroU8;
pub type ClientIdRepr = u8;
pub const MAX_CLIENTS: usize = u8::MAX as _;

pub const PROTOCOL_ID: u16 = 1;
pub const DEFAULT_USER_PROTOCOL_ID: u16 = 0xffff;
