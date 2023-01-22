use glam::IVec3;
use glium::{VertexBuffer, IndexBuffer};
use super::{block::Block, render::ChunkVertex};

pub const CHUNK_SIZE: usize = 32;

pub type BlockData = Box<[[[Block; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]>;

pub struct ChunkData {
  pub blocks: BlockData,
  pub has_renderable_blocks: bool,
}
impl ChunkData {
  pub fn update_metadata(&mut self) {
    todo!()
  }
}

pub struct ChunkMesh {
  pub is_dirty: bool,
  pub vertex_buffer: VertexBuffer<ChunkVertex>,
  pub index_buffer: IndexBuffer<u32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum ChunkState {
  ToUnload,  //desired only
  #[default]
  Nothing,
  Loading,   //current only
  Loaded,
  Meshing,   //current only
  Rendered,
  RecalculatingMesh //current only
}

pub struct Chunk {
  pub position: IVec3,
  pub block_data: Option<ChunkData>,
  pub mesh_index: Option<usize>,
  pub current_state: ChunkState,
  pub desired_state: ChunkState,
}
impl Chunk {
  pub fn new(position: IVec3) -> Self {
    Self {
      position,
      block_data: None,
      mesh_index: None,
      current_state: Default::default(),
      desired_state: Default::default(),
    }
  }
}
