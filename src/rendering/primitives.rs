use glium::{implement_vertex, VertexBuffer, IndexBuffer, index::PrimitiveType};
use shipyard::{NonSendSync, UniqueView, AllStoragesView, Unique};
use super::Renderer;

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

#[derive(Clone, Copy, Default)]
pub struct PositionOnlyVertex {
  pub position: [f32; 3],
}
implement_vertex!(PositionOnlyVertex, position);

const fn box_vertices() -> [PositionOnlyVertex; CUBE_VERTICES.len() / 3] {
  let mut arr = [PositionOnlyVertex { position: [0., 0., 0.] }; CUBE_VERTICES.len() / 3];
  let mut ptr = 0;
  loop {
    arr[ptr] = PositionOnlyVertex {
      position: [
        CUBE_VERTICES[ptr * 3], 
        CUBE_VERTICES[(ptr * 3) + 1], 
        CUBE_VERTICES[(ptr * 3) + 2]
      ]
    };
    ptr += 1;
    if ptr >= CUBE_VERTICES.len() / 3 {
      return arr
    }
  }
}
const BOX_VERTICES: &[PositionOnlyVertex] = &box_vertices();

#[derive(Unique)]
pub struct SimpleBoxBuffers(pub VertexBuffer<PositionOnlyVertex>, pub IndexBuffer<u16>);

pub fn init_simple_box_buffers(
  storages: AllStoragesView,
  display: NonSendSync<UniqueView<Renderer>>
) {
  let vert = VertexBuffer::new(
    &display.display,
    BOX_VERTICES
  ).unwrap();
  let index = IndexBuffer::new(
    &display.display,
    PrimitiveType::TrianglesList, 
    CUBE_INDICES
  ).unwrap();
  storages.add_unique_non_send_sync(SimpleBoxBuffers(vert, index));
}
