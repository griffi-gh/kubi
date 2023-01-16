use std::collections::HashMap;

mod chunk;
use chunk::Chunk;

pub struct World {
  chunks: HashMap<(i32, i32), Chunk>
}
impl World {
  // pub fn update_loaded_chunks(around: ) {

  // }
}
