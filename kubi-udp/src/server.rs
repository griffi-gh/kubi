use std::net::{UdpSocket, SocketAddr};
use crate::BINCODE_CONFIG;

pub struct Server {
  socket: UdpSocket,
}
impl Server {
  pub fn bind(addr: SocketAddr) -> anyhow::Result<Self> {
    let socket = UdpSocket::bind(addr)?;
    socket.set_nonblocking(true)?;
    socket.set_broadcast(true)?;
    Ok(Self { socket })
  }
}
