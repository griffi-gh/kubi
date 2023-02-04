use anyhow::{Result, bail};
use std::{
  net::{UdpSocket, SocketAddr},
  time::{Instant, Duration},
  marker::PhantomData, 
};
use bincode::{Encode, Decode};
use crate::{
  BINCODE_CONFIG, 
  packet::{ClientPacket, IdClientPacket, IdServerPacket, ServerPacket},
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
  pub config: ClientConfig,
  addr: SocketAddr,
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
  pub fn new(addr: SocketAddr, config: ClientConfig) -> Result<Self> {
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
  
  fn send_raw_packet(&self, packet: ClientPacket<S>) -> Result<()> {
    let id_packet = IdClientPacket(self.client_id, packet);
    let bytes = bincode::encode_to_vec(id_packet, BINCODE_CONFIG)?;
    self.socket.send(&bytes)?;
    Ok(())
  }
  
  fn disconnect_inner(&mut self, reason: DisconnectReason, silent: bool) -> Result<()> {
    if !silent {
      self.send_raw_packet(ClientPacket::Disconnect)?;
    }
    self.client_id = None;
    self.status = ClientStatus::Disconnected;
    self.disconnect_reason = reason;
    Ok(())
  }

  fn reset_timeout(&mut self) {
    self.timeout = Instant::now();
  }

  
  pub fn connect(&mut self) -> Result<()> {
    if self.status != ClientStatus::Disconnected {
      bail!("Not Disconnected");
    }
    self.status = ClientStatus::Connecting;
    self.last_heartbeat = Instant::now();
    self.reset_timeout();
    self.socket.connect(self.addr)?;
    self.send_raw_packet(ClientPacket::Connect)?;
    Ok(())
  }

  pub fn disconnect(&mut self) -> Result<()> {
    if self.status != ClientStatus::Connected {
      bail!("Not Connected");
    }
    self.disconnect_inner(DisconnectReason::ClientDisconnected, false)?;
    Ok(())
  }

  pub fn send_message(&self, message: S) -> Result<()> {
    if self.status != ClientStatus::Connected {
      bail!("Not Connected");
    }
    self.send_raw_packet(ClientPacket::Data(message))?;
    Ok(())
  }
  
  pub fn update(&mut self, callback: fn(R) -> Result<()>) -> Result<()> {
    if self.status == ClientStatus::Disconnected {
      return Ok(())
    }
    if self.timeout.elapsed() > self.config.timeout {
      log::warn!("Client timed out");
      //We don't care if this packet actually gets sent because the server is likely dead
      let _ = self.disconnect_inner(DisconnectReason::ClientDisconnected, false).map_err(|_| {
        log::warn!("Failed to send disconnect packet");
      });
      return Ok(())
    }
    if self.last_heartbeat.elapsed() > self.config.heartbeat_interval {
      log::trace!("Sending heartbeat packet");
      self.send_raw_packet(ClientPacket::Heartbeat)?;
      self.last_heartbeat = Instant::now();
    }
    //receive
    let mut buf = Vec::new();
    loop {
      if self.socket.recv(&mut buf).is_ok() {
        //TODO check the first byte of the raw data instead of decoding?
        let (packet, _): (IdServerPacket<R>, _) = bincode::decode_from_slice(&buf, BINCODE_CONFIG)?;
        let IdServerPacket(user_id, packet) = packet;
        if self.client_id.map(|x| Some(x) != user_id).unwrap_or_default() {
          continue
        }
        self.reset_timeout();
        match packet {
          ServerPacket::Connected(client_id) => {
            self.client_id = Some(client_id);
            self.status = ClientStatus::Connected;
            return Ok(())
          },
          ServerPacket::Disconnected(reason) => {
            let reason = DisconnectReason::KickedByServer(reason);
            //this should never fail but we're handling the error anyway
            self.disconnect_inner(reason, true)?;
            return Ok(())
          },
          ServerPacket::Data(message) => {
            callback(message)?;
          }
        }
      } else {
        break
      }
      buf.clear();
    }
    Ok(())
  }
}
