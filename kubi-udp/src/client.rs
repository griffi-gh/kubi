use std::{
  net::{UdpSocket, SocketAddr},
  marker::PhantomData
};
use bincode::{Encode, Decode};
use crate::{BINCODE_CONFIG, packet::ClientPacket};

pub struct Client<S, R> where S: Encode + Decode, R: Encode + Decode {
  socket: UdpSocket,
  _s: PhantomData<S>,
  _r: PhantomData<R>,
}
impl<S, R> Client<S, R> where S: Encode + Decode, R: Encode + Decode {
  pub fn connect(addr: SocketAddr) -> anyhow::Result<Self> {
    let bind_addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
    let socket = UdpSocket::bind(bind_addr)?;
    socket.set_nonblocking(true)?;
    socket.connect(addr)?;
    let client = Self {
      socket,
      _s: PhantomData,
      _r: PhantomData,
    };
    client.send_packet(&ClientPacket::Connect)?;
    Ok(client)
  }
  fn send_packet(&self, packet: &ClientPacket<S>) -> anyhow::Result<()> {
    let bytes = bincode::encode_to_vec(packet, BINCODE_CONFIG)?;
    self.socket.send(&bytes)?;
    Ok(())
  }
  pub fn send_message(&self, message: S) -> anyhow::Result<()> {
    self.send_packet(&ClientPacket::Data(message))?;
    Ok(())
  }
  pub fn disconnect(self) -> anyhow::Result<()> {
    self.send_packet(&ClientPacket::Disconnect)?;
    Ok(())
  }
}
