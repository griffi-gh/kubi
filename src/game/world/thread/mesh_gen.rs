use glam::IVec2;
use crate::game::world::chunk::{Chunk, ChunkData};
use crate::game::shaders::chunk::Vertex as ChunkVertex;

pub fn generate_mesh(position: IVec2, chunk_data: ChunkData, neighbors: [ChunkData; 4]) -> Vec<ChunkVertex> {
  vec![]
}
