use glium::implement_vertex;

#[derive(Clone, Copy)]
pub struct Vertex {
  pub position: [f32; 2]
}
implement_vertex!(Vertex, position);

pub const VERTEX_SHADER: &str = include_str!("./glsl/colored2d.vert");
pub const FRAGMENT_SHADER: &str = include_str!("./glsl/colored2d.frag");
