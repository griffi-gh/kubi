use glium::{Display, Program, implement_vertex};

pub struct Programs {
    pub colored_2d: Program,
}
impl Programs {
    pub fn compile_all(display: &Display) -> Self {
        Self {
            colored_2d: Program::from_source(display, COLORED_2D_VERTEX_SHADER, COLORED_2D_FRAGMENT_SHADER, None).unwrap(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Colored2dVertex {
    pub position: [f32; 2]
}
implement_vertex!(Colored2dVertex, position);

pub const COLORED_2D_VERTEX_SHADER: &str = r#"
    #version 140

    in vec2 position;

    void main() {
        gl_Position = vec4(position, 0., 1.);
    }
"#;
pub const COLORED_2D_FRAGMENT_SHADER: &str = r#"
    #version 140

    out vec4 color;
    uniform vec4 u_color;

    void main() {
        color = u_color;
    }
"#;

//TODO store vertex data in a more compact way
pub const CHUNK_VERTEX_SHADER: &str = r#"
    #version 150

    in vec3 position;
    in vec3 normal;

    out vec3 v_normal;

    uniform mat4 perspective;
    uniform mat4 view;
    uniform mat4 model;

    void main() {
        mat4 modelview = view * model;
        v_normal = transpose(inverse(mat3(modelview))) * normal;
        gl_Position = perspective * modelview * vec4(position, 1.0);
    }
"#;
pub const CHUNK_FRAGMENT_SHADER: &str = r#"
    #version 150
"#;
