use glium::VertexBuffer;
use crate::game::{
  blocks::Block,
  shaders::chunk::Vertex as ChunkVertex
};

pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_HEIGHT: usize = 255;

pub enum ChunkState {
  //AwaitsLoading,
  //Loaded,
  //AwaitsMesh,
  //Rendered,
  //AwaitsUnload
}

pub struct Chunk {
  pub coords: (i32, i32),
  pub block_data: Option<[[[Block; CHUNK_SIZE]; CHUNK_HEIGHT]; CHUNK_SIZE]>,
  pub vertex_buffer: Option<VertexBuffer<ChunkVertex>>,
  pub state: ChunkState
}
