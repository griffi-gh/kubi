use std::{
  net::{UdpSocket, SocketAddr},
  marker::PhantomData
};
use bincode::{Encode, Decode};
use crate::{BINCODE_CONFIG, packet::ClientPacket};

pub struct Client<T> where T: Encode + Decode {
  socket: UdpSocket,
  _marker: PhantomData<T>
}
impl<T> Client<T> where T: Encode + Decode {
  pub fn connect(addr: SocketAddr) -> anyhow::Result<Self> {
    let bind_addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
    let socket = UdpSocket::bind(bind_addr)?;
    socket.set_nonblocking(true)?;
    socket.connect(addr)?;
    Ok(Self {
      socket,
      _marker: PhantomData
    })
  }
  fn send_packet(&self, packet: &ClientPacket<T>) -> anyhow::Result<()> {
    let bytes = bincode::encode_to_vec(packet, BINCODE_CONFIG)?;
    self.socket.send(&bytes)?;
    Ok(())
  }
  pub fn send(&self, message: T) -> anyhow::Result<()> {
    self.send_packet(&ClientPacket::Data(message))?;
    Ok(())
  }
  pub fn disconnect(&self) -> anyhow::Result<()> {
    self.send_packet(&ClientPacket::Disconnect)?;
    Ok(())
  }
}
