use bincode::{Encode, Decode};
use crate::chunk::BlockData;

type IVec3Arr = [i32; 3];
type Vec3Arr = [f32; 3];
type QuatArr = [f32; 3];

pub const PROTOCOL_ID: u16 = 1;

#[derive(Encode, Decode, Clone)]
pub enum ClientToServerMessage {
  ClientHello {
    username: String,
    password: Option<String>,
  },
  PositionChanged {
    position: Vec3Arr,
    velocity: Vec3Arr,
    direction: QuatArr,
  },
  ChunkRequest {
    chunk: IVec3Arr,
  },
}

#[derive(Encode, Decode, Clone)]
pub struct InitData {
  
}

#[derive(Encode, Decode, Clone)]
pub enum ServerToClientMessage {
  ServerHello {
    init: InitData
  },
  ServerFuckOff {
    reason: String,
  },
  PlayerPositionChanged {
    client_id: u8,
    position: Vec3Arr,
    direction: QuatArr,
  },
  ChunkResponse {
    chunk: IVec3Arr,
    data: BlockData
  }
}
