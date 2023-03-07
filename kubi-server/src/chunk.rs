use glam::IVec3;
use hashbrown::HashMap;
use kubi_shared::chunk::BlockData;
use shipyard::Unique;

pub struct Chunk {
  pub blocks: BlockData
}

#[derive(Unique)]
pub struct ChunkManager {
  pub chunks: HashMap<IVec3, Chunk>
}

pub fn server_chunk_response(
  
) {

}
