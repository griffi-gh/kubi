use glam::{Vec3, IVec3, Quat};
use serde::{Serialize, Deserialize};
use crate::{chunk::BlockData, queue::QueuedBlock};
use super::client::ClientId;

pub const PROTOCOL_ID: u16 = 2;

pub const C_CLIENT_HELLO: u8 = 0;
pub const C_POSITION_CHANGED: u8 = 1;
pub const C_CHUNK_SUB_REQUEST: u8 = 2;

#[derive(Serialize, Deserialize, Clone)]
#[repr(u8)]
pub enum ClientToServerMessage {
  ClientHello {
    username: String,
    password: Option<String>,
  } = C_CLIENT_HELLO,
  PositionChanged {
    position: Vec3,
    velocity: Vec3,
    direction: Quat,
  } = C_POSITION_CHANGED,
  ChunkSubRequest {
    chunk: IVec3,
  } = C_CHUNK_SUB_REQUEST,
}

pub const S_SERVER_HELLO: u8 = 0;
pub const S_SERVER_FUCK_OFF: u8 = 1;
pub const S_PLAYER_POSITION_CHANGED: u8 = 2;
pub const S_CHUNK_RESPONSE: u8 = 3;

#[derive(Serialize, Deserialize, Clone)]
#[repr(u8)]
pub enum ServerToClientMessage {
  ServerHello {
    init: InitData
  } = S_SERVER_HELLO,
  ServerFuckOff {
    reason: String,
  } = S_SERVER_FUCK_OFF,
  PlayerPositionChanged {
    client_id: u8,
    position: Vec3,
    direction: Quat,
  } = S_PLAYER_POSITION_CHANGED,
  ChunkResponse {
    chunk: IVec3,
    data: BlockData,
    queued: Vec<QueuedBlock>,
  } = S_CHUNK_RESPONSE,
}

//---

#[derive(Serialize, Deserialize, Clone)]
pub struct UserInitData {
  pub client_id: ClientId,
  pub username: String,
  pub position: Vec3,
  pub velocity: Vec3,
  pub direction: Quat,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct InitData {
  pub users: Vec<UserInitData>
}
