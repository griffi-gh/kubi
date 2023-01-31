use std::net::{UdpSocket, SocketAddr};
use super::messages::ClientToServerMessage;

use crate::BINCODE_CONFIG;

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
    let bytes = bincode::encode_to_vec(message, BINCODE_CONFIG)?;
    self.socket.send(&bytes)?;
    Ok(())
  }
}
