use glam::{Vec3, Quat};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum ClientToServerMessage {
  ClientHello {
    username: String,
    password: Option<String>,
  },
  PositionChanged {
    position: Vec3,
    direction: Quat
  }
}

#[derive(Serialize, Deserialize)]
pub enum ServerToClientMessage {
  ServerHello {
    client_id: u16,
  },
  ServerFuckOff {
    reason: String,
  },
  PlayerPositionChanged {

  },
}
