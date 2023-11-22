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

struct BufferPair {
  vertex_buffer: glium::VertexBuffer<Vertex>,
  index_buffer: glium::IndexBuffer<u32>,
  vertex_count: usize,
  index_count: usize,
}

impl BufferPair {
  pub fn new<F: Facade>(facade: &F) -> Self {
    log::debug!("init ui buffers...");
    Self {
      vertex_buffer: VertexBuffer::empty_persistent(facade, 1024).unwrap(),
      index_buffer: IndexBuffer::empty_persistent(facade, PrimitiveType::TrianglesList, 1024).unwrap(),
      vertex_count: 0,
      index_count: 0,
    }
  }

  pub fn ensure_buffer_size(&mut self, need_vtx: usize, need_idx: usize) {
    let current_vtx_size = self.vertex_buffer.get_size();
    let current_idx_size = self.index_buffer.get_size();
    if current_vtx_size >= need_vtx && current_idx_size >= need_idx {
      return
    }
    let new_vtx_size = (need_vtx + 1).next_power_of_two();
    let new_idx_size = (need_idx + 1).next_power_of_two();
    log::debug!("resizing buffers: vtx {} -> {}, idx {} -> {}", current_vtx_size, new_vtx_size, current_idx_size, new_idx_size);
    if current_vtx_size != new_vtx_size {
      self.vertex_buffer = VertexBuffer::empty_persistent(
        self.vertex_buffer.get_context(),
        new_vtx_size
      ).unwrap();
    }
    if current_idx_size != new_idx_size {
      self.index_buffer = IndexBuffer::empty_persistent(
        self.index_buffer.get_context(),
        PrimitiveType::TrianglesList,
        new_idx_size
      ).unwrap();
    }
  }

  pub fn write_data(&mut self, vtx: &[Vertex], idx: &[u32]) {
    log::debug!("uploading {} vertices and {} indices", vtx.len(), idx.len());

    self.vertex_count = vtx.len();
    self.index_count = idx.len();

    self.vertex_buffer.invalidate();
    self.index_buffer.invalidate();

    if self.vertex_count == 0 || self.index_count == 0 {
      return
    }

    self.ensure_buffer_size(vtx.len(), idx.len());
    self.vertex_buffer.slice_mut(0..vtx.len()).unwrap().write(vtx);
    self.index_buffer.slice_mut(0..idx.len()).unwrap().write(idx);
  }

  pub fn is_empty(&self) -> bool {
    self.vertex_count == 0 || self.index_count == 0
  }
}

pub struct GliumUiRenderer {
  program: glium::Program,
  buffer: BufferPair,
}

impl GliumUiRenderer {
  pub fn new<F: Facade>(facade: &F) -> Self {
    log::info!("init glium backend for ui");
    log::debug!("init program");
    let program = Program::from_source(facade, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap();
    Self {
      program,
      buffer: BufferPair::new(facade)
    }
  }

  pub fn update(&mut self, plan: &UiDrawPlan) {
    assert!(plan.calls.len() == 1, "multiple draw calls not supported yet");
    let data_vtx = &plan.calls[0].vertices.iter().copied().map(Vertex::from).collect::<Vec<_>>();
    let data_idx = &plan.calls[0].indices;
    self.buffer.write_data(data_vtx, data_idx);
  }

  pub fn draw(&self, frame: &mut glium::Frame, resolution: Vec2) {
    if self.buffer.is_empty() {
      return
    }

    let params = DrawParameters {
      blend: Blend::alpha_blending(),
      ..Default::default()
    };

    frame.draw(
      self.buffer.vertex_buffer.slice(0..self.buffer.vertex_count).unwrap(),
      self.buffer.index_buffer.slice(0..self.buffer.index_count).unwrap(),
      &self.program,
      &uniform! {
        resolution: resolution.to_array(),
      },
      &params,
    ).unwrap();
  }
}
