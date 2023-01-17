use glam::IVec2;
use glium::{VertexBuffer, IndexBuffer};
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

pub type ChunkData = Box<[[[Block; CHUNK_SIZE]; CHUNK_HEIGHT]; CHUNK_SIZE]>;

pub struct ChunkMesh {
  pub is_dirty: bool,
  pub vertex_buffer: VertexBuffer<ChunkVertex>,
  pub index_buffer: IndexBuffer<u32>,
}

pub struct Chunk {
  pub position: IVec2,
  pub block_data: Option<ChunkData>,
  pub mesh: Option<ChunkMesh>,
  pub state: ChunkState,
  pub desired: ChunkState,
}
impl Chunk {
  pub fn new(position: IVec2) -> Self {
    Self {
      position,
      block_data: None,
      mesh: None,
      state: ChunkState::Nothing,
      desired: ChunkState::Nothing,
    }
  }
}
