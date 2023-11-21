use hashbrown::{HashMap, HashSet};
use nohash_hasher::BuildNoHashHasher;
use std::any::Any;

#[derive(Default)]
pub struct StateRepo {
  state: HashMap<u64, Box<dyn Any>, BuildNoHashHasher<u64>>,
  active_ids: HashSet<u64, BuildNoHashHasher<u64>>
}
