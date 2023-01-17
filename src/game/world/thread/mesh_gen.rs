use glam::{IVec2, IVec3, Vec2, Vec3A, vec3a, vec2, ivec3};
use strum::{EnumIter, IntoEnumIterator};
use crate::game::{
  world::{
    POSITIVE_X_NEIGHBOR,
    NEGATIVE_X_NEIGHBOR,
    POSITIVE_Z_NEIGHBOR,
    NEGATIVE_Z_NEIGHBOR,
    chunk::{ChunkData, CHUNK_SIZE, CHUNK_HEIGHT}
  },
  shaders::chunk::Vertex, 
  blocks::Block
};

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
pub const CUBE_FACE_NORMALS: [[f32; 3]; 6] = [
  [0.,  1., 0.],
  [0.,  0., -1.],
  [-1., 0., 0.],
  [1.,  0., 0.],
  [0.,  0., 1.],
  [0., -1., 0.]
];
pub const CUBE_FACE_INDICES: [u32; 6] = [0, 1, 2, 2, 1, 3];

#[derive(Default)]
struct MeshBuilder {
  vertex_buffer: Vec<Vertex>,
  index_buffer: Vec<u32>,
  idx_counter: u32,
}
impl MeshBuilder {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn add_face(&mut self, face: CubeFace, coord: IVec3, uvs: [Vec2; 4]) {
    let coord = coord.as_vec3a();
    let face_index = face as usize;
    
    //Push vertexes
    let norm = CUBE_FACE_NORMALS[face_index];
    let vert = CUBE_FACE_VERTICES[face_index];
    self.vertex_buffer.reserve(4);
    for i in 0..4 {
      self.vertex_buffer.push(Vertex {
        position: (coord + vert[i]).to_array(),
        normal: norm,
        uv: uvs[i].to_array()
      });
    }

    //Push indices
    self.index_buffer.extend_from_slice(&CUBE_FACE_INDICES.map(|x| x + self.idx_counter));
    self.idx_counter += 4;
  }

  pub fn finish(self) -> (Vec<Vertex>, Vec<u32>) {
    (self.vertex_buffer, self.index_buffer)
  }
}

pub fn generate_mesh(position: IVec2, chunk_data: ChunkData, neighbors: [ChunkData; 4]) -> (Vec<Vertex>, Vec<u32>) {
  let get_block = |pos: IVec3| -> Block {
    if pos.x < 0 {
      neighbors[NEGATIVE_X_NEIGHBOR][(CHUNK_SIZE as i32 + pos.x) as usize][pos.y as usize][pos.z as usize]
    } else if pos.x >= CHUNK_SIZE as i32 {
      neighbors[POSITIVE_X_NEIGHBOR][pos.x as usize - CHUNK_SIZE as usize][pos.y as usize][pos.z as usize]
    } else if pos.z < 0 {
      neighbors[NEGATIVE_Z_NEIGHBOR][pos.x as usize][pos.y as usize][(CHUNK_SIZE as i32 + pos.z) as usize]
    } else if pos.z >= CHUNK_SIZE as i32 {
      neighbors[POSITIVE_Z_NEIGHBOR][pos.x as usize][pos.y as usize][pos.z as usize - CHUNK_SIZE as usize]
    } else {
      chunk_data[pos.x as usize][pos.y as usize][pos.z as usize]
    }
  };
  
  let mut builer = MeshBuilder::new();

  for x in 0..CHUNK_SIZE {
    for y in 0..CHUNK_HEIGHT {
      for z in 0..CHUNK_SIZE {
        let coord = ivec3(x as i32, y as i32, z as i32);
        let descriptor = get_block(coord).descriptor();
        if descriptor.render.is_none() {
          continue
        }
        for face in CubeFace::iter() {
          let facing = Vec3A::from_array(CUBE_FACE_NORMALS[face as usize]).as_ivec3();
          let facing_coord = coord + facing;
          let show = {
            (facing_coord.y < 0) || 
            (facing_coord.y >= CHUNK_HEIGHT as i32) || 
            get_block(facing_coord).descriptor().render.is_none()
          };
          if show {
            let texures = descriptor.render.unwrap().1;
            let texture_id = match face {
              CubeFace::Top    => texures.top,
              CubeFace::Front  => texures.front,
              CubeFace::Left   => texures.left,
              CubeFace::Right  => texures.right,
              CubeFace::Back   => texures.back,
              CubeFace::Bottom => texures.bottom,
            };
            //TODO replace with a proper texture resolver (or calculate uvs in a shader!)
            //this is temporary!
            //also this can only resolve textures on the first row.

            const TEX_WIDTH: f32 = 16. / 640.;
            const TEX_HEIGHT: f32 = 16. / 404.;
            let x1 = TEX_WIDTH * texture_id as f32;
            let x2 = x1 + TEX_WIDTH as f32;
            let y1 = 1. - TEX_HEIGHT;
            let y2 = 1.;
            builer.add_face(face, coord, [
              vec2(x1, y1),
              vec2(x1, y2),
              vec2(x2, y1),
              vec2(x2, y2),
            ]);
          }
        }
      }
    }
  }
  
  builer.finish()
}
