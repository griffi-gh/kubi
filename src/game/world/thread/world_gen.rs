use crate::game::{
  world::chunk::{ChunkData, CHUNK_SIZE, CHUNK_HEIGHT}, 
  blocks::Block
};

pub fn generate_chunk() -> ChunkData {
  Box::new([[[Block::Stone; CHUNK_SIZE]; CHUNK_HEIGHT]; CHUNK_SIZE])
}
