use strum::EnumIter;
use glam::{Vec3, vec3, IVec3, ivec3};
use crate::rendering::world::ChunkVertex;

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
impl CubeFace {
  pub const fn normal(self) -> IVec3 {
    CUBE_FACE_NORMALS_IVEC3[self as usize]
  }
}

const CUBE_FACE_VERTICES: [[Vec3; 4]; 6] = [
  [vec3(0., 1., 0.), vec3(0., 1., 1.), vec3(1., 1., 0.), vec3(1., 1., 1.)],
  [vec3(0., 0., 0.), vec3(0., 1., 0.), vec3(1., 0., 0.), vec3(1., 1., 0.)],
  [vec3(0., 0., 1.), vec3(0., 1., 1.), vec3(0., 0., 0.), vec3(0., 1., 0.)],
  [vec3(1., 0., 0.), vec3(1., 1., 0.), vec3(1., 0., 1.), vec3(1., 1., 1.)],
  [vec3(1., 0., 1.), vec3(1., 1., 1.), vec3(0., 0., 1.), vec3(0., 1., 1.)],
  [vec3(0., 0., 1.), vec3(0., 0., 0.), vec3(1., 0., 1.), vec3(1., 0., 0.)],
];
const CUBE_FACE_NORMALS_IVEC3: [IVec3; 6] = [ 
  ivec3( 0,  1,  0),
  ivec3( 0,  0, -1),
  ivec3(-1,  0,  0),
  ivec3( 1,  0,  0),
  ivec3( 0,  0,  1),
  ivec3( 0, -1,  0)
];
const CUBE_FACE_NORMALS: [Vec3; 6] = [ 
  vec3(0., 1., 0.),
  vec3(0., 0., -1.),
  vec3(-1.,0., 0.),
  vec3(1., 0., 0.),
  vec3(0., 0., 1.),
  vec3(0., -1.,0.)
];
const CUBE_FACE_INDICES: [u32; 6] = [0, 1, 2, 2, 1, 3];
const UV_COORDS: [[f32; 2]; 4] = [
  [0., 0.],
  [0., 1.],
  [1., 0.],
  [1., 1.],
];

#[derive(Default)]
pub struct MeshBuilder {
  vertex_buffer: Vec<ChunkVertex>,
  index_buffer: Vec<u32>,
  idx_counter: u32,
}
impl MeshBuilder {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn add_face(&mut self, face: CubeFace, coord: IVec3, texture: u8) {
    let coord = coord.as_vec3();
    let face_index = face as usize;
    
    //Push vertices
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
