use glium::implement_vertex;

#[derive(Clone, Copy)]
pub struct ChunkVertex {
  pub position: [f32; 3],
  pub normal: [f32; 3],
  pub uv: [f32; 2],
  pub tex_index: u8,
}
implement_vertex!(ChunkVertex, position, normal, uv, tex_index);
