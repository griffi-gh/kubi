pub const CUBE_VERTICES: &[f32] = &[
  // front
  0.0, 0.0, 1.0,
  1.0, 0.0, 1.0,
  1.0, 1.0, 1.0,
  0.0, 1.0, 1.0,
  // back
  0.0, 0.0, 0.0,
  1.0, 0.0, 0.0,
  1.0, 1.0, 0.0,
  0.0, 1.0, 0.0
];
pub const CUBE_INDICES: &[u16] = &[
  // front
  0, 1, 2,
  2, 3, 0,
  // right
  1, 5, 6,
  6, 2, 1,
  // back
  7, 6, 5,
  5, 4, 7,
  // left
  4, 0, 3,
  3, 7, 4,
  // bottom
  4, 5, 1,
  1, 0, 4,
  // top
  3, 2, 6,
  6, 7, 3
];
