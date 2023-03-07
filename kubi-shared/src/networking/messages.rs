use std::num::NonZeroUsize;
use bincode::{Encode, Decode};
use crate::{chunk::BlockData, block::Block};

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
  ChunkSubRequest {
    chunk: IVec3Arr,
  },
}

#[derive(Encode, Decode, Clone)]
pub struct UserInitData {
  pub client_id: NonZeroUsize, //maybe use the proper type instead
  pub username: String,
  pub position: Vec3Arr,
  pub velocity: Vec3Arr,
  pub direction: QuatArr,
}

#[derive(Encode, Decode, Clone)]
pub struct InitData {
  pub users: Vec<UserInitData>
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
    data: BlockData,
    queued: Vec<(IVec3Arr, Block)>,
  }
}
