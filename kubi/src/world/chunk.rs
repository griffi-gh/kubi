use glam::IVec3;
use glium::{VertexBuffer, IndexBuffer};
use crate::rendering::world::ChunkVertex;

pub use kubi_shared::chunk::{CHUNK_SIZE, BlockData};

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
impl DesiredChunkState {
  pub fn matches_current(self, current: CurrentChunkState) -> bool {
    (matches!(self, DesiredChunkState::Nothing)  && matches!(current, CurrentChunkState::Nothing)) ||
    (matches!(self, DesiredChunkState::Loaded)   && matches!(current, CurrentChunkState::Loaded)) ||
    (matches!(self, DesiredChunkState::Rendered) && matches!(current, CurrentChunkState::Rendered))
  }
}

pub struct Chunk {
  pub position: IVec3,
  pub block_data: Option<ChunkData>,
  pub mesh_index: Option<usize>,
  pub current_state: CurrentChunkState,
  pub desired_state: DesiredChunkState,
  pub mesh_dirty: bool,
}
impl Chunk {
  pub fn new(position: IVec3) -> Self {
    Self {
      position,
      block_data: None,
      mesh_index: None,
      current_state: Default::default(),
      desired_state: Default::default(),
      mesh_dirty: false,
    }
  }
}
