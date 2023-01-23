use strum::{EnumIter, IntoEnumIterator};
use glam::{Vec3A, vec3a};

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
const CUBE_FACE_NORMALS: [Vec3A; 6] = [ //this is likely incorrect for a right handed system
  vec3a(0., 1., 0.),
  vec3a(0., 0., 1.),
  vec3a(-1.,0., 0.),
  vec3a(1., 0., 0.),
  vec3a(0., 0., -1.),
  vec3a(0., -1.,0.)
];
const CUBE_FACE_INDICES: [u32; 6] = [0, 1, 2, 2, 1, 3];
const UV_COORDS: [[f32; 2]; 4] = [
  [0., 0.],
  [0., 1.],
  [1., 0.],
  [1., 1.],
];

pub fn generate_mesh(data: MeshGenData) {
  
}
