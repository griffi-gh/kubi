use std::rc::Rc;
use glam::Vec2;
use glium::{
  Surface, DrawParameters, Blend,
  Program, VertexBuffer, IndexBuffer,
  backend::{Facade, Context},
  texture::{SrgbTexture2d, RawImage2d},
  index::PrimitiveType,
  implement_vertex,
  uniform, uniforms::{Sampler, SamplerBehavior, SamplerWrapFunction},
};
use kubi_ui::{
  KubiUi,
  draw::{UiDrawPlan, UiVertex, BindTexture},
  text::FontTextureInfo, IfModified,
};

const VERTEX_SHADER: &str = include_str!("../shaders/vertex.vert");
const FRAGMENT_SHADER: &str = include_str!("../shaders/fragment.frag");
const FRAGMENT_SHADER_TEX: &str = include_str!("../shaders/fragment_tex.frag");

#[derive(Clone, Copy)]
#[repr(C)]
struct Vertex {
  position: [f32; 2],
  color: [f32; 4],
  uv: [f32; 2],
}

impl From<UiVertex> for Vertex {
  fn from(v: UiVertex) -> Self {
    Self {
      position: v.position.to_array(),
      color: v.color.to_array(),
      uv: v.uv.to_array(),
    }
  }
}

implement_vertex!(Vertex, position, color, uv);

struct BufferPair {
  pub vertex_buffer: glium::VertexBuffer<Vertex>,
  pub index_buffer: glium::IndexBuffer<u32>,
  pub vertex_count: usize,
  pub index_count: usize,
}

impl BufferPair {
  pub fn new<F: Facade>(facade: &F) -> Self {
    log::debug!("init ui buffers...");
    Self {
      vertex_buffer: VertexBuffer::empty_dynamic(facade, 1024).unwrap(),
      index_buffer: IndexBuffer::empty_dynamic(facade, PrimitiveType::TrianglesList, 1024).unwrap(),
      vertex_count: 0,
      index_count: 0,
    }
  }

  pub fn ensure_buffer_size(&mut self, need_vtx: usize, need_idx: usize) {
    let current_vtx_size = self.vertex_buffer.get_size() / std::mem::size_of::<Vertex>();
    let current_idx_size = self.index_buffer.get_size() / std::mem::size_of::<u32>();
    //log::debug!("current vtx size: {}, current idx size: {}", current_vtx_size, current_idx_size);
    if current_vtx_size >= need_vtx && current_idx_size >= need_idx {
      return
    }
    let new_vtx_size = (need_vtx + 1).next_power_of_two();
    let new_idx_size = (need_idx + 1).next_power_of_two();
    log::debug!("resizing buffers: vtx {} -> {}, idx {} -> {}", current_vtx_size, new_vtx_size, current_idx_size, new_idx_size);
    if current_vtx_size != new_vtx_size {
      self.vertex_buffer = VertexBuffer::empty_dynamic(
        self.vertex_buffer.get_context(),
        new_vtx_size
      ).unwrap();
    }
    if current_idx_size != new_idx_size {
      self.index_buffer = IndexBuffer::empty_dynamic(
        self.index_buffer.get_context(),
        PrimitiveType::TrianglesList,
        new_idx_size
      ).unwrap();
    }
  }

  pub fn write_data(&mut self, vtx: &[Vertex], idx: &[u32]) {
    //log::trace!("uploading {} vertices and {} indices", vtx.len(), idx.len());

    self.vertex_count = vtx.len();
    self.index_count = idx.len();

    self.vertex_buffer.invalidate();
    self.index_buffer.invalidate();

    if self.vertex_count == 0 || self.index_count == 0 {
      return
    }

    self.ensure_buffer_size(self.vertex_count, self.index_count);

    self.vertex_buffer.slice_mut(0..self.vertex_count).unwrap().write(vtx);
    self.index_buffer.slice_mut(0..self.index_count).unwrap().write(idx);
  }

  pub fn is_empty(&self) -> bool {
    self.vertex_count == 0 || self.index_count == 0
  }
}

struct GlDrawCall {
  active: bool,
  buffer: BufferPair,
  bind_texture: Option<Rc<SrgbTexture2d>>,
}

pub struct GliumUiRenderer {
  context: Rc<Context>,
  program: glium::Program,
  program_tex: glium::Program,
  font_texture: Option<Rc<SrgbTexture2d>>,
  plan: Vec<GlDrawCall>,
}

impl GliumUiRenderer {
  pub fn new<F: Facade>(facade: &F) -> Self {
    log::info!("init glium backend for kui");
    Self {
      program: Program::from_source(facade, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap(),
      program_tex: Program::from_source(facade, VERTEX_SHADER, FRAGMENT_SHADER_TEX, None).unwrap(),
      context: Rc::clone(facade.get_context()),
      font_texture: None,
      plan: vec![]
    }
  }

  pub fn update_draw_plan(&mut self, plan: &UiDrawPlan) {
    if plan.calls.len() > self.plan.len() {
      self.plan.resize_with(plan.calls.len(), || {
        GlDrawCall {
          buffer: BufferPair::new(&self.context),
          bind_texture: None,
          active: false,
        }
      });
    } else {
      for step in &mut self.plan[plan.calls.len()..] {
        step.active = false;
      }
    }
    for (idx, call) in plan.calls.iter().enumerate() {
      let data_vtx = &call.vertices.iter().copied().map(Vertex::from).collect::<Vec<_>>()[..];
      let data_idx = &call.indices[..];
      self.plan[idx].active = true;
      self.plan[idx].buffer.write_data(data_vtx, data_idx);
      self.plan[idx].bind_texture = match call.bind_texture {
        Some(BindTexture::FontTexture) => {
          const NO_FNT_TEX: &str = "Font texture exists in draw plan but not yet inited. Make sure to call update_font_texture() *before* update_draw_plan()";
          Some(Rc::clone(self.font_texture.as_ref().expect(NO_FNT_TEX)))
        },
        None => None,
      }
    }
  }

  pub fn update_font_texture(&mut self, font_texture: &FontTextureInfo) {
    log::debug!("updating font texture");
    self.font_texture = Some(Rc::new(SrgbTexture2d::new(
      &self.context,
      RawImage2d::from_raw_rgba(
        font_texture.data.to_owned(),
        (font_texture.size.x, font_texture.size.y)
      )
    ).unwrap()));
  }

  pub fn update(&mut self, kui: &KubiUi) {
    if let Some(texture) = kui.font_texture().if_modified() {
      self.update_font_texture(texture);
    }
    if let Some(plan) = kui.draw_plan().if_modified() {
      self.update_draw_plan(plan);
    }
  }

  pub fn draw(&self, frame: &mut glium::Frame, resolution: Vec2) {
    let params = DrawParameters {
      blend: Blend::alpha_blending(),
      ..Default::default()
    };

    for step in &self.plan {
      if !step.active {
        continue
      }

      if step.buffer.is_empty() {
        continue
      }

      let vtx_buffer = step.buffer.vertex_buffer.slice(0..step.buffer.vertex_count).unwrap();
      let idx_buffer = step.buffer.index_buffer.slice(0..step.buffer.index_count).unwrap();

      if let Some(bind_texture) = step.bind_texture.as_ref() {
        frame.draw(
          vtx_buffer,
          idx_buffer,
          &self.program_tex,
          &uniform! {
            resolution: resolution.to_array(),
            tex: Sampler(bind_texture.as_ref(), SamplerBehavior {
              wrap_function: (SamplerWrapFunction::Clamp, SamplerWrapFunction::Clamp, SamplerWrapFunction::Clamp),
              ..Default::default()
            }),
          },
          &params,
        ).unwrap();
      } else {
        frame.draw(
          vtx_buffer,
          idx_buffer,
          &self.program,
          &uniform! {
            resolution: resolution.to_array(),
          },
          &params,
        ).unwrap();
      }
    }
  }
}
