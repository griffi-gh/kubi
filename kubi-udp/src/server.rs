use std::{
  net::{UdpSocket, SocketAddr},
  time::Instant,
  marker::PhantomData,
  collections::{VecDeque, vec_deque::Drain as DrainDeque},
  io::ErrorKind
};
use anyhow::{Result, bail};
use bincode::{Encode, Decode};
use hashbrown::HashMap;
use nohash_hasher::BuildNoHashHasher;
use crate::{
  BINCODE_CONFIG,
  common::{ClientId, ClientIdRepr, MAX_CLIENTS},
  packet::{IdClientPacket, ClientPacket, ServerPacket, IdServerPacket}
};

//i was feeling a bit sick while writing most of this please excuse me for my terrible code :3

pub struct ConnectedClient {
  id: ClientId,
  addr: SocketAddr,
  timeout: Instant,
}

#[derive(Clone, Copy, Debug)]
pub struct ServerConfig {
  pub max_clients: usize,
}
impl Default for ServerConfig {
  fn default() -> Self {
    Self {
      max_clients: MAX_CLIENTS,
    }
  }
}

pub enum ServerEvent<T> where T: Encode + Decode {
  Connected(ClientId),
  Disconnected(ClientId),
  MessageReceived {
    from: ClientId,
    message: T
  }
}

pub struct Server<S, R> where S: Encode + Decode, R: Encode + Decode {
  socket: UdpSocket,
  clients: HashMap<ClientId, ConnectedClient, BuildNoHashHasher<ClientIdRepr>>,
  config: ServerConfig,
  event_queue: VecDeque<ServerEvent<R>>,
  _s: PhantomData<S>,
}
impl<S, R> Server<S, R> where S: Encode + Decode, R: Encode + Decode {
  pub fn bind(addr: SocketAddr, config: ServerConfig) -> anyhow::Result<Self> {
    assert!(config.max_clients <= MAX_CLIENTS);
    let socket = UdpSocket::bind(addr)?;
    socket.set_nonblocking(true)?;
    Ok(Self { 
      config,
      socket,
      clients: HashMap::with_capacity_and_hasher(MAX_CLIENTS, BuildNoHashHasher::default()),
      event_queue: VecDeque::new(),
      _s: PhantomData,
    })
  }

  fn send_to_addr(&self, addr: SocketAddr, packet: IdServerPacket<S>) -> Result<()> {
    let bytes = bincode::encode_to_vec(packet, BINCODE_CONFIG)?;
    self.socket.send_to(&bytes, addr)?;
    Ok(())
  }

  fn send_packet(&self, packet: IdServerPacket<S>) -> Result<()> {
    let Some(id) = packet.0 else {
      bail!("send_to_client call without id")
    };
    let Some(client) = self.clients.get(&id) else {
      bail!("client with id {id} doesn't exist")
    };
    self.send_to_addr(client.addr, packet)?;
    Ok(())
  }

  fn add_client(&mut self, addr: SocketAddr) -> Result<ClientId> {
    let Some(id) = (1..=self.config.max_clients)
      .map(|x| ClientId::new(x as _).unwrap())
      .find(|i| !self.clients.contains_key(i)) else {
        bail!("Server full");
      };
    if self.clients.iter().any(|x| x.1.addr == addr) {
      bail!("Already connected from the same address");
    }
    self.clients.insert(id, ConnectedClient {
      id,
      addr,
      timeout: Instant::now(),
    });
    Ok(id)
  }

  fn disconnect_client_inner(&mut self, id: ClientId, reason: String) -> Result<()> {
    let result = self.send_packet(IdServerPacket(
      Some(id), ServerPacket::Disconnected(reason)
    ));
    self.clients.remove(&id);
    result
  }

  pub fn kick_client(&mut self, id: ClientId, reason: String) -> Result<()> {
    if !self.clients.contains_key(&id) {
      bail!("Already disconnected")
    }
    self.disconnect_client_inner(id, reason)?;
    Ok(())
  }

  pub fn shutdown(mut self) -> Result<()> {
    let clients = self.clients.keys().copied().collect::<Vec<ClientId>>();
    for id in clients {
      self.kick_client(id, "Server is shutting down".into())?;
    }
    Ok(())
  }

  pub fn send_message(&mut self, id: ClientId, message: S) -> anyhow::Result<()> {
    self.send_packet(IdServerPacket(Some(id), ServerPacket::Data(message)))?;
    Ok(())
  }
  
  pub fn update(&mut self) -> Result<()> {
    //TODO client timeout
    let mut buf = [0; u16::MAX as usize];
    loop {
      match self.socket.recv_from(&mut buf) {
        Ok((len, addr)) => {
          if let Ok(packet) = bincode::decode_from_slice(&buf[..len], BINCODE_CONFIG) {
            let (packet, _): (IdClientPacket<R>, _) = packet;
            let IdClientPacket(id, packet) = packet;
            match id {
              Some(id) => {
                if !self.clients.contains_key(&id) {
                  bail!("Client with id {id} doesn't exist");
                };
                match packet {
                  ClientPacket::Data(data) => {
                    self.event_queue.push_back(ServerEvent::MessageReceived {
                      from: id,
                      message: data,
                    });
                  }
                  ClientPacket::Disconnect => {
                    log::info!("Client {id} disconnected");
                    self.event_queue.push_back(ServerEvent::Disconnected(id));
                    self.disconnect_client_inner(id, "Disconnected".into())?;
                  },
                  ClientPacket::Heartbeat => {
                    self.clients.get_mut(&id).unwrap().timeout = Instant::now()
                  },
                  ClientPacket::Connect => bail!("Client already connected"),
                }
              },
              None => {
                match packet {
                  ClientPacket::Connect => {
                    match self.add_client(addr) {
                      Ok(id) => {
                        log::info!("Client with id {id} connected");
                        self.event_queue.push_back(ServerEvent::Connected(id));
                        self.send_to_addr(addr, 
                          IdServerPacket(None, ServerPacket::Connected(id)
                        ))?;
                      },
                      Err(error) => {
                        let reason = error.to_string();
                        log::error!("Client connection failed: {reason}");
                        self.send_to_addr(addr, IdServerPacket(
                          None, ServerPacket::Disconnected(reason)
                        ))?;
                      }
                    }
                  },
                  _ => bail!("Invalid packet type for non-id packet")
                }
              }
            }
          } else {
            bail!("Corrupted packet received");
          }
        },
        Err(error) if error.kind() != ErrorKind::WouldBlock => {
          log::warn!("IO error {}", error);
          // return Err(error.into());
        },
        _ => break,
      }
    }
    Ok(())
  }

  pub fn pop_event(&mut self) -> Option<ServerEvent<R>> {
    self.event_queue.pop_front()
  }
  pub fn process_events(&mut self) -> DrainDeque<ServerEvent<R>> {
    self.event_queue.drain(..)
  }
}
