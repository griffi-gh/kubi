use bincode::{Encode, Decode};
use crate::chunk::BlockData;

type IVec3Arr = [i32; 3];
type Vec3Arr = [f32; 3];
type QuatArr = [f32; 3];

#[derive(Encode, Decode)]
pub enum ClientToServerMessage {
  ClientHello {
    username: String,
    password: Option<String>,
  },
  PositionChanged {
    client_id: u8,
    secret: u32,
    position: Vec3Arr,
    direction: QuatArr,
  },
  ChunkRequest {
    client_id: u8,
    secret: u32,
    chunk: IVec3Arr,
  },
}

#[derive(Encode, Decode)]
pub enum ServerToClientMessage {
  ServerHello {
    client_id: u8,
    secret: u32,
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
