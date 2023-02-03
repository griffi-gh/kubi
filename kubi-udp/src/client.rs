use std::{
  net::{UdpSocket, SocketAddr},
  marker::PhantomData, time::{Instant, Duration}
};
use bincode::{Encode, Decode};
use crate::{
  BINCODE_CONFIG, 
  packet::{ClientPacket, IdClientPacket},
  common::{ClientId, DisconnectReason}
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClientStatus {
  Disconnected,
  Connecting,
  Connected,
}

#[derive(Clone, Copy, Debug)]
pub struct ClientConfig {
  pub timeout: Duration,
  pub heartbeat_interval: Duration,
}

pub struct Client<S, R> where S: Encode + Decode, R: Encode + Decode {
  addr: SocketAddr,
  config: ClientConfig,
  socket: UdpSocket,
  status: ClientStatus,
  timeout: Instant,
  last_heartbeat: Instant,
  client_id: Option<ClientId>,
  disconnect_reason: DisconnectReason,
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
      timeout: Instant::now(),
      last_heartbeat: Instant::now(),
      client_id: None,
      disconnect_reason: DisconnectReason::default(),
      _s: PhantomData,
      _r: PhantomData,
    })
  }
  pub fn connect(&mut self) -> anyhow::Result<()> {
    if self.status != ClientStatus::Disconnected {
      anyhow::bail!("Not Disconnected");
    }
    self.status = ClientStatus::Connecting;
    self.timeout = Instant::now();
    self.last_heartbeat = Instant::now();
    self.socket.connect(self.addr)?;
    self.send_raw_packet(ClientPacket::Connect)?;
    Ok(())
  }
  fn send_raw_packet(&self, packet: ClientPacket<S>) -> anyhow::Result<()> {
    let id_packet = IdClientPacket(self.client_id, packet);
    let bytes = bincode::encode_to_vec(id_packet, BINCODE_CONFIG)?;
    self.socket.send(&bytes)?;
    Ok(())
  }
  pub fn send_message(&self, message: S) -> anyhow::Result<()> {
    self.send_raw_packet(ClientPacket::Data(message))?;
    Ok(())
  }
  fn disconnect_inner(&mut self, reason: DisconnectReason) -> anyhow::Result<()> {
    self.send_raw_packet(ClientPacket::Disconnect)?;
    self.status = ClientStatus::Disconnected;
    self.disconnect_reason = DisconnectReason::ClientDisconnected;
    Ok(())
  }
  pub fn disconnect(&mut self) -> anyhow::Result<()> {
    if self.status != ClientStatus::Connected {
      anyhow::bail!("Not Connected");
    }
    self.disconnect_inner(DisconnectReason::ClientDisconnected)?;
    Ok(())
  }
  pub fn update(&mut self) -> anyhow::Result<()> {
    if self.status == ClientStatus::Disconnected {
      return Ok(())
    }
    if self.timeout.elapsed() > self.config.timeout {
      log::warn!("Client timed out");
      //We don't care if this packet actually gets sent because the server is likely dead
      let _ = self.disconnect_inner(DisconnectReason::ClientDisconnected).map_err(|_| {
        log::warn!("Failed to send disconnect packet");
      });
      return Ok(())
    }
    if self.last_heartbeat.elapsed() > self.config.heartbeat_interval {
      log::trace!("Sending heartbeat packet");
      self.send_raw_packet(ClientPacket::Heartbeat)?;
      self.last_heartbeat = Instant::now();
    }
    Ok(())
  }
}
