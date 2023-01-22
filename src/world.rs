use nohash_hasher::BuildNoHashHasher;
use shipyard::Unique;
use glam::{IVec3, ivec3};
use hashbrown::HashMap;
use anyhow::{Result, Context};

pub mod chunk;
pub mod block;
pub mod render;
pub mod tasks;
pub mod loading;
pub mod mesh;

use chunk::{Chunk, ChunkMesh};

//TODO separate world struct for render data
// because this is not send-sync

pub struct AllChunksNeighbors<'a> {
  pub center: &'a Chunk,
  pub top:    &'a Chunk,
  pub bottom: &'a Chunk,
  pub left:   &'a Chunk,
  pub right:  &'a Chunk,
  pub front:  &'a Chunk,
  pub back:   &'a Chunk,
}
pub struct AllChunksNeighborsMut<'a> {
  pub center: &'a mut Chunk,
  pub top:    &'a mut Chunk,
  pub bottom: &'a mut Chunk,
  pub left:   &'a mut Chunk,
  pub right:  &'a mut Chunk,
  pub front:  &'a mut Chunk,
  pub back:   &'a mut Chunk,
}
pub struct ChunksNeighbors<'a> {
  pub center: Option<&'a Chunk>,
  pub top:    Option<&'a Chunk>,
  pub bottom: Option<&'a Chunk>,
  pub left:   Option<&'a Chunk>,
  pub right:  Option<&'a Chunk>,
  pub front:  Option<&'a Chunk>,
  pub back:   Option<&'a Chunk>,
}
impl<'a> ChunksNeighbors<'a> {
  pub fn all(&self) -> Option<AllChunksNeighbors<'a>> {
    Some(AllChunksNeighbors {
      center: self.center?,
      top:    self.top?,
      bottom: self.bottom?,
      left:   self.left?,
      right:  self.right?,
      front:  self.front?,
      back:   self.back?,
    })
  }
}

#[derive(Default, Unique)]
#[track(Modification)]
pub struct ChunkStorage {
  pub chunks: HashMap<IVec3, Chunk>
}
impl ChunkStorage {
  pub fn new() -> Self {
    Self::default()
  }
  pub fn neighbors(&self, coords: IVec3) -> ChunksNeighbors {
    ChunksNeighbors {
      center: self.chunks.get(&coords),
      top:    self.chunks.get(&(coords - ivec3(0, 1, 0))),
      bottom: self.chunks.get(&(coords + ivec3(0, 1, 0))),
      left:   self.chunks.get(&(coords - ivec3(1, 0, 0))),
      right:  self.chunks.get(&(coords + ivec3(1, 0, 0))),
      front:  self.chunks.get(&(coords - ivec3(0, 0, 1))),
      back:   self.chunks.get(&(coords + ivec3(0, 0, 1))),
    }
  }
  pub fn neighbors_all(&self, coords: IVec3) -> Option<AllChunksNeighbors> {
    self.neighbors(coords).all()
  }
  pub fn neighbors_all_mut(&mut self, coords: IVec3) -> Option<AllChunksNeighborsMut> {
    let mut refs = self.chunks.get_many_mut([
      &coords,
      &(coords - ivec3(0, 1, 0)),
      &(coords + ivec3(0, 1, 0)),
      &(coords - ivec3(1, 0, 0)),
      &(coords + ivec3(1, 0, 0)),
      &(coords - ivec3(0, 0, 1)),
      &(coords + ivec3(0, 0, 1)),
    ])?.map(Some);
    Some(AllChunksNeighborsMut {
      center: std::mem::take(&mut refs[0]).unwrap(),
      top:    std::mem::take(&mut refs[1]).unwrap(),
      bottom: std::mem::take(&mut refs[2]).unwrap(),
      left:   std::mem::take(&mut refs[3]).unwrap(),
      right:  std::mem::take(&mut refs[4]).unwrap(),
      front:  std::mem::take(&mut refs[5]).unwrap(),
      back:   std::mem::take(&mut refs[6]).unwrap(),
    })
  }
}


#[derive(Default, Unique)]
pub struct ChunkMeshStorage {
  meshes: HashMap<usize, ChunkMesh, BuildNoHashHasher<usize>>,
  index: usize,
}
impl ChunkMeshStorage {
  pub fn new() -> Self {
    Self {
      meshes: HashMap::with_capacity_and_hasher(250, BuildNoHashHasher::default()),
      index: 0,
    }
  }
  pub fn insert(&mut self, mesh: ChunkMesh) -> usize {
    let index = self.index;
    self.meshes.insert_unique_unchecked(index, mesh);
    self.index += 1;
    index
  }
  pub fn update(&mut self, key: usize, mesh: ChunkMesh) -> Result<()> {
    *self.meshes.get_mut(&key).context("Chunk doesn't exist")? = mesh;
    Ok(())
  }
  pub fn remove(&mut self, key: usize) -> Result<()> {
    self.meshes.remove(&key).context("Chunk doesn't exist")?;
    Ok(())
  }
}
