use std::num::NonZeroU8;

pub type ClientId = NonZeroU8;
pub type ClientIdRepr = u8;
pub const MAX_CLIENTS: usize = u8::MAX as _;
