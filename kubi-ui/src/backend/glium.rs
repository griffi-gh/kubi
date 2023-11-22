use glam::Vec2;
use glium::{
  Surface, DrawParameters, Blend,
  Program, VertexBuffer, IndexBuffer,
  backend::Facade,
  index::PrimitiveType,
  implement_vertex, uniform,
};

use crate::draw::{UiDrawPlan, UiVertex};

const VERTEX_SHADER: &str = include_str!("../../shaders/vertex.vert");
const FRAGMENT_SHADER: &str = include_str!("../../shaders/fragment.frag");

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

  fn ensure_buffer_size(&mut self, need_vtx: usize, need_idx: usize) {
    let current_vtx_size = self.vertex_buffer.get_size();
    let current_idx_size = self.index_buffer.get_size();
    if current_vtx_size >= need_vtx && current_idx_size >= need_idx {
      return
    }
    let new_vtx_size = (need_vtx + 1).next_power_of_two();
    let new_idx_size = (need_idx + 1).next_power_of_two();
    log::debug!("resizing buffers: vtx {} -> {}, idx {} -> {}", current_vtx_size, new_vtx_size, current_idx_size, new_idx_size);
    if current_vtx_size != new_vtx_size {
      self.vertex_buffer = VertexBuffer::empty_persistent(self.vertex_buffer.get_context(), new_vtx_size).unwrap();
    }
    if current_idx_size != new_idx_size {
      self.index_buffer = IndexBuffer::empty_persistent(self.index_buffer.get_context(), PrimitiveType::TrianglesList, new_idx_size).unwrap();
    }
  }

  fn write_buffer_data(&mut self, vtx: &[Vertex], idx: &[u32]) {
    self.ensure_buffer_size(vtx.len(), idx.len());
    self.vertex_buffer.invalidate();
    self.vertex_buffer.slice_mut(0..vtx.len()).unwrap().write(vtx);
    self.index_buffer.invalidate();
    self.index_buffer.slice_mut(0..idx.len()).unwrap().write(idx);
  }

  pub fn update(&mut self, plan: &UiDrawPlan) {
    let data_vtx = &plan.vertices.iter().copied().map(Vertex::from).collect::<Vec<_>>();
    let data_idx = &plan.indices;
    self.write_buffer_data(data_vtx, data_idx);
  }

  pub fn draw(&self, frame: &mut glium::Frame, resolution: Vec2) {
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
