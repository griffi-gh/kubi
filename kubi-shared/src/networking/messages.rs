use glam::{Vec3, Quat};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum ClientToServerMessage {
  ClientHello {
    username: String,
    password: Option<String>,
  },
  PositionChanged {
    client_id: u8,
    secret: u32,
    position: Vec3,
    direction: Quat
  }
}

#[derive(Serialize, Deserialize)]
pub enum ServerToClientMessage {
  ServerHello {
    client_id: u8,
    secret: u32,
  },
  ServerFuckOff {
    reason: String,
  },
  PlayerPositionChanged {

  },
}
