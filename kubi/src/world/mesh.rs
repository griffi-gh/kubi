use strum::{EnumIter, IntoEnumIterator};
use glam::{Vec3A, vec3a, IVec3, ivec3};
use std::mem::discriminant;
use kubi_shared::block::{Block, RenderType};
use super::{chunk::CHUNK_SIZE, };
use crate::rendering::world::ChunkVertex;

pub mod data;
use data::MeshGenData;

#[repr(usize)]
#[derive(Clone, Copy, Debug, EnumIter)]
pub enum CubeFace {
  Top    = 0,
  Front  = 1,
  Left   = 2,
  Right  = 3,
  Back   = 4,
  Bottom = 5,
}
const CUBE_FACE_VERTICES: [[Vec3A; 4]; 6] = [
  [vec3a(0., 1., 0.), vec3a(0., 1., 1.), vec3a(1., 1., 0.), vec3a(1., 1., 1.)],
  [vec3a(0., 0., 0.), vec3a(0., 1., 0.), vec3a(1., 0., 0.), vec3a(1., 1., 0.)],
  [vec3a(0., 0., 1.), vec3a(0., 1., 1.), vec3a(0., 0., 0.), vec3a(0., 1., 0.)],
  [vec3a(1., 0., 0.), vec3a(1., 1., 0.), vec3a(1., 0., 1.), vec3a(1., 1., 1.)],
  [vec3a(1., 0., 1.), vec3a(1., 1., 1.), vec3a(0., 0., 1.), vec3a(0., 1., 1.)],
  [vec3a(0., 0., 1.), vec3a(0., 0., 0.), vec3a(1., 0., 1.), vec3a(1., 0., 0.)],
];
const CUBE_FACE_NORMALS: [Vec3A; 6] = [ 
  vec3a(0., 1., 0.),
  vec3a(0., 0., -1.),
  vec3a(-1.,0., 0.),
  vec3a(1., 0., 0.),
  vec3a(0., 0., 1.),
  vec3a(0., -1.,0.)
];
const CUBE_FACE_INDICES: [u32; 6] = [0, 1, 2, 2, 1, 3];
const UV_COORDS: [[f32; 2]; 4] = [
  [0., 0.],
  [0., 1.],
  [1., 0.],
  [1., 1.],
];

#[derive(Default)]
struct MeshBuilder {
  vertex_buffer: Vec<ChunkVertex>,
  index_buffer: Vec<u32>,
  idx_counter: u32,
}
impl MeshBuilder {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn add_face(&mut self, face: CubeFace, coord: IVec3, texture: u8) {
    let coord = coord.as_vec3a();
    let face_index = face as usize;
    
    //Push vertexes
    let norm = CUBE_FACE_NORMALS[face_index];
    let vert = CUBE_FACE_VERTICES[face_index];
    self.vertex_buffer.reserve(4);
    for i in 0..4 {
      self.vertex_buffer.push(ChunkVertex {
        position: (coord + vert[i]).to_array(),
        normal: norm.to_array(),
        uv: UV_COORDS[i], 
        tex_index: texture
      });
    }

    //Push indices
    self.index_buffer.extend_from_slice(&CUBE_FACE_INDICES.map(|x| x + self.idx_counter));
    self.idx_counter += 4;
  }

  pub fn finish(self) -> (Vec<ChunkVertex>, Vec<u32>) {
    (self.vertex_buffer, self.index_buffer)
  }
}

pub fn generate_mesh(data: MeshGenData) -> (Vec<ChunkVertex>, Vec<u32>) {
  let get_block = |pos: IVec3| -> Block {
    if pos.x < 0 {
      data.block_data_neg_x[(CHUNK_SIZE as i32 + pos.x) as usize][pos.y as usize][pos.z as usize]
    } else if pos.x >= CHUNK_SIZE as i32 {
      data.block_data_pos_x[pos.x as usize - CHUNK_SIZE][pos.y as usize][pos.z as usize]
    } else if pos.y < 0 {
      data.block_data_neg_y[pos.x as usize][(CHUNK_SIZE as i32 + pos.y) as usize][pos.z as usize]
    } else if pos.y >= CHUNK_SIZE as i32 {
      data.block_data_pos_y[pos.x as usize][pos.y as usize - CHUNK_SIZE][pos.z as usize]
    } else if pos.z < 0 {
      data.block_data_neg_z[pos.x as usize][pos.y as usize][(CHUNK_SIZE as i32 + pos.z) as usize]
    } else if pos.z >= CHUNK_SIZE as i32 {
      data.block_data_pos_z[pos.x as usize][pos.y as usize][pos.z as usize - CHUNK_SIZE]
    } else {
      data.block_data[pos.x as usize][pos.y as usize][pos.z as usize]
    }
  };

  let mut builder = MeshBuilder::new();

  for x in 0..CHUNK_SIZE {
    for y in 0..CHUNK_SIZE {
      for z in 0..CHUNK_SIZE {
        let coord = ivec3(x as i32, y as i32, z as i32);
        let block = get_block(coord);
        let descriptor = block.descriptor();
        if matches!(descriptor.render, RenderType::None) {
          continue
        }
        for face in CubeFace::iter() {
          let facing = CUBE_FACE_NORMALS[face as usize].as_ivec3();
          let facing_coord = coord + facing;
          let show = discriminant(&get_block(facing_coord).descriptor().render) != discriminant(&descriptor.render);
          if show {
            match descriptor.render {
              RenderType::SolidBlock(textures) => {
                let face_texture = match face {
                  CubeFace::Top    => textures.top,
                  CubeFace::Front  => textures.front,
                  CubeFace::Left   => textures.left,
                  CubeFace::Right  => textures.right,
                  CubeFace::Back   => textures.back,
                  CubeFace::Bottom => textures.bottom,
                };
                builder.add_face(face, coord, face_texture as u8);
              },
              _ => unimplemented!()
            }
          }
        }
      }
    }
  }

  builder.finish()
}
