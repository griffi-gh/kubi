use glam::IVec2;
use glium::VertexBuffer;
use crate::game::{
  blocks::Block,
  shaders::chunk::Vertex as ChunkVertex
};

pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_HEIGHT: usize = 255;

pub enum ChunkState {
  Unload,
  Nothing,
  Loading,
  Loaded,
  Rendering,
  Rendered,
}

pub type ChunkData = [[[Block; CHUNK_SIZE]; CHUNK_HEIGHT]; CHUNK_SIZE];
pub type ChunkMesh = VertexBuffer<ChunkVertex>;

pub struct Chunk {
  pub position: IVec2,
  pub block_data: Option<ChunkData>,
  pub vertex_buffer: Option<(bool, ChunkMesh)>,
  pub state: ChunkState,
  pub desired: ChunkState,
}
impl Chunk {
  pub fn new(position: IVec2) -> Self {
    Self {
      position,
      block_data: None,
      vertex_buffer: None,
      state: ChunkState::Nothing,
      desired: ChunkState::Nothing,
    }
  }
}
