use glam::Vec2;
use glium::{
  Surface, DrawParameters, Blend,
  Program, VertexBuffer, IndexBuffer,
  backend::Facade,
  index::PrimitiveType,
  implement_vertex, uniform,
};

use crate::draw::{UiDrawPlan, UiVertex};

const VERTEX_SHADER: &str = include_str!("../../shaders/fragment.frag");
const FRAGMENT_SHADER: &str = include_str!("../../shaders/vertex.vert");

#[derive(Clone, Copy)]
struct Vertex {
  position: [f32; 2],
  color: [f32; 4],
}

impl From<UiVertex> for Vertex {
  fn from(v: UiVertex) -> Self {
    Self {
      position: v.position.to_array(),
      color: v.color.to_array(),
    }
  }
}

implement_vertex!(Vertex, position, color);

pub struct GliumUiRenderer {
  pub program: glium::Program,
  pub vertex_buffer: glium::VertexBuffer<Vertex>,
  pub index_buffer: glium::IndexBuffer<u32>,
}

impl GliumUiRenderer {
  pub fn new<F: Facade>(facade: &F) -> Self {
    log::info!("init glium backend for ui");
    log::debug!("init program");
    let program = Program::from_source(facade, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap();
    log::debug!("init buffers");
    let vertex_buffer = VertexBuffer::empty_persistent(facade, 1024).unwrap();
    let index_buffer = IndexBuffer::empty_persistent(facade, PrimitiveType::TrianglesList, 1024).unwrap();
    Self {
      program,
      vertex_buffer,
      index_buffer,
    }
  }

  pub fn draw(&mut self, frame: &mut glium::Frame, resolution: Vec2, plan: &UiDrawPlan) {
    self.vertex_buffer.write(&plan.vertices.iter().copied().map(Vertex::from).collect::<Vec<_>>());
    self.index_buffer.write(&plan.indices);

    let params = DrawParameters {
      blend: Blend::alpha_blending(),
      ..Default::default()
    };

    frame.draw(
      &self.vertex_buffer,
      &self.index_buffer,
      &self.program,
      &uniform! {
        resolution: resolution.to_array(),
      },
      &params,
    ).unwrap();
  }
}
