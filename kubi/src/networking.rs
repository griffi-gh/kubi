use std::net::SocketAddr;

use shipyard::Unique;

#[derive(Unique, Clone, Copy, PartialEq, Eq)]
pub enum GameType {
  Singleplayer,
  Muliplayer
}

#[derive(Unique, Clone, Copy, PartialEq, Eq)]
pub struct ServerAddress(pub SocketAddr);
