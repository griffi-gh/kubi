use glam::{Vec3, IVec3, Quat};
use serde::{Serialize, Deserialize};
use crate::{chunk::{BlockData, CHUNK_SIZE}, queue::QueuedBlock, entity::Health};
use super::client::ClientId;

pub const PROTOCOL_ID: u16 = 1;

pub const C_CLIENT_HELLO: u8 = 0;
pub const C_POSITION_CHANGED: u8 = 1;
pub const C_CHUNK_SUB_REQUEST: u8 = 2;
pub const C_CHUNK_UNUBSCRIBE: u8 = 3;
pub const C_QUEUE_BLOCK: u8 = 4;

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
  ChunkUnsubscribe {
    chunk: IVec3,
  } = C_CHUNK_UNUBSCRIBE,
  QueueBlock {
    item: QueuedBlock
  } = C_QUEUE_BLOCK,
}

pub const S_SERVER_HELLO: u8 = 0;
pub const S_SERVER_FUCK_OFF: u8 = 1;
pub const S_PLAYER_POSITION_CHANGED: u8 = 2;
pub const S_CHUNK_RESPONSE: u8 = 3;
pub const S_QUEUE_BLOCK: u8 = 4;
pub const S_PLAYER_CONNECTED: u8 = 5;
pub const S_PLAYER_DISCONNECTED: u8 = 6;

#[serde_with::serde_as]
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
    client_id: ClientId,
    position: Vec3,
    direction: Quat,
  } = S_PLAYER_POSITION_CHANGED,

  ///## WARNING: THIS IS COMPRESSED
  ///MESSAGES OF THIS TYPE ARE FULLY
  ///COMPRESSED ***EXCEPT THE FIRST BYTE***
  ///TO REDUCE NETWORK USAGE
  ChunkResponse {
    chunk: IVec3,
    #[serde_as(as = "Box<[[[_; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]>")]
    data: BlockData,
    queued: Vec<QueuedBlock>,
  } = S_CHUNK_RESPONSE,

  QueueBlock {
    item: QueuedBlock
  } = S_QUEUE_BLOCK,

  PlayerConnected {
    init: ClientInitData
  } = S_PLAYER_CONNECTED,

  PlayerDisconnected {
    id: ClientId
  } = S_PLAYER_DISCONNECTED,
}

//---

#[derive(Serialize, Deserialize, Clone)]
pub struct ClientInitData {
  pub client_id: ClientId,
  pub username: String,
  pub position: Vec3,
  pub velocity: Vec3,
  pub direction: Quat,
  pub health: Health,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct InitData {
  pub user: ClientInitData,
  pub users: Vec<ClientInitData>
}
