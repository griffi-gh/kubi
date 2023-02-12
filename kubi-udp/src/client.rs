use anyhow::{Result, bail};
use std::{
  net::{UdpSocket, SocketAddr},
  time::{Instant, Duration},
  marker::PhantomData, 
  collections::{VecDeque, vec_deque::Drain as DrainDeque}, 
  io::ErrorKind,
};
use bincode::{Encode, Decode};
use crate::{
  BINCODE_CONFIG, 
  packet::{ClientPacket, IdClientPacket, IdServerPacket, ServerPacket},
  common::ClientId
};

#[derive(Default, Clone, Debug)]
#[repr(u8)]
pub enum DisconnectReason {
  #[default]
  NotConnected,
  ClientDisconnected,
  KickedByServer(Option<String>),
  Timeout,
}

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
impl Default for ClientConfig {
  fn default() -> Self {
    Self {
      timeout: Duration::from_secs(5),
      heartbeat_interval: Duration::from_secs(3),
    }
  }
}

pub enum ClientEvent<T> where T: Encode + Decode {
  Connected(ClientId),
  Disconnected(DisconnectReason),
  MessageReceived(T)
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
  event_queue: VecDeque<ClientEvent<R>>,
  _s: PhantomData<S>,
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
      event_queue: VecDeque::new(),
      _s: PhantomData,
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
    self.event_queue.push_back(ClientEvent::Disconnected(self.disconnect_reason.clone()));
    Ok(())
  }

  fn reset_timeout(&mut self) {
    self.timeout = Instant::now();
  }

  
  pub fn connect(&mut self) -> Result<()> {
    log::info!("client connect called");
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
  
  pub fn update(&mut self) -> Result<()> { // , callback: fn(ClientEvent<R>) -> Result<()>
    if self.status == ClientStatus::Disconnected {
      return Ok(())
    }
    if self.timeout.elapsed() > self.config.timeout {
      log::warn!("Client timed out");
      //We don't care if this packet actually gets sent because the server is likely dead
      let _ = self.disconnect_inner(DisconnectReason::Timeout, false).map_err(|_| {
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
    match self.socket.recv(&mut buf) {
      Ok(_) => {
        //TODO check the first byte of the raw data instead of decoding?
        let (packet, _): (IdServerPacket<R>, _) = bincode::decode_from_slice(&buf, BINCODE_CONFIG)?;
        let IdServerPacket(user_id, packet) = packet;
        if self.client_id.map(|x| Some(x) != user_id).unwrap_or_default() {
          return Ok(())
        }
        self.reset_timeout();
        match packet {
          ServerPacket::Connected(client_id) => {
            log::info!("client connected with id {client_id}");
            self.client_id = Some(client_id);
            self.status = ClientStatus::Connected;
            self.event_queue.push_back(ClientEvent::Connected(client_id));
            return Ok(())
          },
          ServerPacket::Disconnected(reason) => {
            log::info!("client kicked: {reason}");
            let reason = DisconnectReason::KickedByServer(Some(reason));
            self.disconnect_inner(reason, true)?; //this should never fail but we're handling the error anyway 
            return Ok(())
          },
          ServerPacket::Data(message) => {
            self.event_queue.push_back(ClientEvent::MessageReceived(message));
          }
        }
      },
      Err(error) if error.kind() != ErrorKind::WouldBlock => {
        return Err(error.into());
      },
      _ => (),
    }
    Ok(())
  }

  pub fn get_event(&mut self) -> Option<ClientEvent<R>> {
    self.event_queue.pop_front()
  }
  pub fn process_events(&mut self) -> DrainDeque<ClientEvent<R>> {
    self.event_queue.drain(..)
  }
}
