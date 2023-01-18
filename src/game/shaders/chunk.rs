use glium::implement_vertex;

#[derive(Clone, Copy)]
pub struct Vertex {
  pub position: [f32; 3],
  pub normal: [f32; 3],
  pub uv: [f32; 2],
  pub tex_index: u8,
}
implement_vertex!(Vertex, position, normal, uv, tex_index);

pub const VERTEX_SHADER: &str = include_str!("./glsl/chunk.vert");
pub const FRAGMENT_SHADER: &str = include_str!("./glsl/chunk.frag");

// pub const VERTEX_SHADER: &str = r#"
//   #version 150 core

//   in vec3 position;
//   in vec3 normal;
//   in vec2 uv;
//   out vec3 v_normal;
//   out vec2 v_uv;
//   uniform mat4 perspective;
//   uniform mat4 view;
//   uniform mat4 model;

//   void main() {
//     mat4 modelview = view * model;
//     //v_normal = transpose(inverse(mat3(modelview))) * normal;
//     v_normal = normal;
//     v_uv = uv;
//     gl_Position = perspective * modelview * vec4(position, 1.0);
//   }
// "#;
