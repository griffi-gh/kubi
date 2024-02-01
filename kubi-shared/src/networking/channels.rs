#[repr(u8)]
pub enum Channel {
  Generic = 0,
  Auth = 1,
  World = 2,
  Block = 3,
  Move = 4,
  SysEvt = 5,
}
