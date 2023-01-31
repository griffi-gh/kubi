use std::net::{UdpSocket, SocketAddr};
use super::messages::ClientToServerMessage;

const BINCODE_CONFIG: bincode::config::Configuration<bincode::config::LittleEndian, bincode::config::Varint, bincode::config::SkipFixedArrayLength> = bincode::config::standard()
  .with_little_endian()
  .with_variable_int_encoding()
  .skip_fixed_array_length();

pub struct Client {
  socket: UdpSocket
}
impl Client {
  pub fn bind(addr: SocketAddr) -> anyhow::Result<Self> {
    Ok(Self {
      socket: UdpSocket::bind(addr)?
    })
  }
  pub fn send(&self, message: ClientToServerMessage) -> anyhow::Result<()> {
    let bytes = bincode::serde::encode_to_vec(message, BINCODE_CONFIG)?;
    self.socket.send(&bytes)?;
    Ok(())
  }
}
