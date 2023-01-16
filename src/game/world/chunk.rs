use glam::IVec2;
use glium::VertexBuffer;
use crate::game::{
  blocks::Block,
  shaders::chunk::Vertex as ChunkVertex
};

pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_HEIGHT: usize = 255;

pub enum ChunkState {
  AwaitsLoading,
  Loaded,
  AwaitsMesh,
  Rendered,
  AwaitsUnload
}

pub enum DesiredState {
  Unloaded,
  Loaded,
  Rendered,
}

pub type ChunkData = [[[Block; CHUNK_SIZE]; CHUNK_HEIGHT]; CHUNK_SIZE];
pub type ChunkMesh = VertexBuffer<ChunkVertex>;

pub struct Chunk {
  pub position: IVec2,
  pub block_data: Option<ChunkData>,
  pub vertex_buffer: Option<ChunkMesh>,
  pub state: ChunkState,
  pub desired: DesiredState,
}
impl Chunk {
  pub fn new(position: IVec2) -> Self {
    Self {
      position,
      block_data: None,
      vertex_buffer: None,
      state: ChunkState::AwaitsLoading,
      desired: DesiredState::Loaded,
    }
  }
}
