use std::{
  net::{UdpSocket, SocketAddr},
  marker::PhantomData, time::Instant
};
use bincode::{Encode, Decode};
use crate::{BINCODE_CONFIG, packet::ClientPacket};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClientStatus {
  Disconnected,
  Connecting,
  Connected,
}

#[derive(Clone, Copy, Debug)]
pub struct ClientConfig {
  
}

pub struct Client<S, R> where S: Encode + Decode, R: Encode + Decode {
  addr: SocketAddr,
  config: ClientConfig,
  socket: UdpSocket,
  status: ClientStatus,
  last_heartbeat: Instant,
  _s: PhantomData<*const S>,
  _r: PhantomData<*const R>,
}
impl<S, R> Client<S, R> where S: Encode + Decode, R: Encode + Decode {
  pub fn new(addr: SocketAddr, config: ClientConfig) -> anyhow::Result<Self> {
    let bind_addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
    let socket = UdpSocket::bind(bind_addr)?;
    socket.set_nonblocking(true)?;
    Ok(Self {
      addr,
      config,
      socket,
      status: ClientStatus::Disconnected,
      last_heartbeat: Instant::now(),
      _s: PhantomData,
      _r: PhantomData,
    })
  }
  pub fn connect(&mut self) -> anyhow::Result<()> {
    if self.status != ClientStatus::Disconnected {
      anyhow::bail!("Already {:?}", self.status);
    }
    self.status = ClientStatus::Connecting;
    self.socket.connect(self.addr)?;
    self.send_raw_packet(&ClientPacket::Connect)?;
    Ok(())
  }
  fn send_raw_packet(&self, packet: &ClientPacket<S>) -> anyhow::Result<()> {
    let bytes = bincode::encode_to_vec(packet, BINCODE_CONFIG)?;
    self.socket.send(&bytes)?;
    Ok(())
  }
  pub fn send_message(&self, message: S) -> anyhow::Result<()> {
    self.send_raw_packet(&ClientPacket::Data(message))?;
    Ok(())
  }
  pub fn disconnect(&self) -> anyhow::Result<()> {
    self.send_raw_packet(&ClientPacket::Disconnect)?;
    Ok(())
  }
  pub fn update(&mut self) {
    
  }
}
