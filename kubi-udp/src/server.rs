use std::{net::{UdpSocket, SocketAddr}, time::Instant};
use hashbrown::HashMap;
use nohash_hasher::BuildNoHashHasher;
use crate::{BINCODE_CONFIG, common::{ClientId, MAX_CLIENTS}};

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

pub struct Server {
  socket: UdpSocket,
  clients: HashMap<ClientId, ConnectedClient, BuildNoHashHasher<u8>>,
  config: ServerConfig,
}
impl Server {
  pub fn bind(addr: SocketAddr, config: ServerConfig) -> anyhow::Result<Self> {
    assert!(config.max_clients <= MAX_CLIENTS);
    let socket = UdpSocket::bind(addr)?;
    socket.set_nonblocking(true)?;
    //socket.set_broadcast(true)?;
    Ok(Self { 
      config,
      socket,
      clients: HashMap::with_capacity_and_hasher(MAX_CLIENTS, BuildNoHashHasher::default())
    })
  }
  /// Returns None if there are no free spots left
  fn connect_client(&mut self) -> Option<ClientId> {
    let id = (1..=self.config.max_clients)
      .map(|x| ClientId::new(x as _).unwrap())
      .find(|i| self.clients.contains_key(i))?;
    self.clients.insert(id, ConnectedClient {
      id,
      addr: "0.0.0.0:0".parse().unwrap(),
      timeout: Instant::now(),
    });
    todo!();
    Some(id)
  }
  pub fn update(&mut self) {
    let mut buf = Vec::new();
    if self.socket.recv(&mut buf).is_ok() {
      todo!()
    }
  }
}
