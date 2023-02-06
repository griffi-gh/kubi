use std::{net::{UdpSocket, SocketAddr}, time::Instant};
use hashbrown::HashMap;
use nohash_hasher::BuildNoHashHasher;
use crate::{BINCODE_CONFIG, common::{ClientId, MAX_CLIENTS}};

pub struct ConnectedClient {
  id: ClientId,
  addr: SocketAddr,
  timeout: Instant,
}

pub struct Server {
  socket: UdpSocket,
  clients: HashMap<ClientId, ConnectedClient, BuildNoHashHasher<ClientId>>
}
impl Server {
  pub fn bind(addr: SocketAddr) -> anyhow::Result<Self> {
    let socket = UdpSocket::bind(addr)?;
    socket.set_nonblocking(true)?;
    socket.set_broadcast(true)?;
    Ok(Self { 
      socket,
      clients: HashMap::with_capacity_and_hasher(MAX_CLIENTS, BuildNoHashHasher::default())
    })
  }
}
