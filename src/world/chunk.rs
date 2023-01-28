use glam::IVec3;
use glium::{VertexBuffer, IndexBuffer};
use super::block::Block;
use crate::rendering::world::ChunkVertex;

pub const CHUNK_SIZE: usize = 32;

pub type BlockData = Box<[[[Block; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]>;

pub struct ChunkData {
  pub blocks: BlockData,
  //pub has_renderable_blocks: bool,
}
impl ChunkData {
  // pub fn update_metadata(&mut self) {
  //   todo!()
  // }
}

pub struct ChunkMesh {
  pub vertex_buffer: VertexBuffer<ChunkVertex>,
  pub index_buffer: IndexBuffer<u32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CurrentChunkState {
  #[default]
  Nothing,
  Loading,
  Loaded,
  CalculatingMesh,
  Rendered,
  RecalculatingMesh, 
  Unloading,      
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DesiredChunkState {
  #[default]
  Nothing,
  Loaded,
  Rendered,
  ToUnload,
}

pub struct Chunk {
  pub position: IVec3,
  pub block_data: Option<ChunkData>,
  pub mesh_index: Option<usize>,
  pub current_state: CurrentChunkState,
  pub desired_state: DesiredChunkState,
  pub dirty: bool,
}
impl Chunk {
  pub fn new(position: IVec3) -> Self {
    Self {
      position,
      block_data: None,
      mesh_index: None,
      current_state: Default::default(),
      desired_state: Default::default(),
      dirty: false,
    }
  }
}
