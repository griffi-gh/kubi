use std::collections::HashMap;
use crate::game::chunk::Chunk;

pub struct World {
  chunks: HashMap<(i32, i32), Chunk>
}
