use shipyard::{Unique, AllStoragesView, NonSendSync, UniqueView};
use glium::{VertexBuffer, IndexBuffer, index::PrimitiveType};
use crate::rendering::Renderer;
use super::PositionOnlyVertex2d;

#[derive(Unique)]
pub struct RectPrimitive(pub VertexBuffer<PositionOnlyVertex2d>, pub IndexBuffer<u16>);

const RECT_VERTEX: &[PositionOnlyVertex2d] = &[
  PositionOnlyVertex2d { position: [0., 0.] },
  PositionOnlyVertex2d { position: [1., 0.] },
  PositionOnlyVertex2d { position: [0., 1.] },
  PositionOnlyVertex2d { position: [1., 1.] },
];
const RECT_INDEX: &[u16] = &[0, 1, 2, 1, 3, 2];

pub(super) fn init_rect_primitive(
  storages: AllStoragesView,
  display: NonSendSync<UniqueView<Renderer>>
) {
  let vert = VertexBuffer::immutable(
    &display.display,
    RECT_VERTEX
  ).unwrap();
  let index = IndexBuffer::immutable(
    &display.display,
    PrimitiveType::TrianglesList,
    RECT_INDEX
  ).unwrap();
  storages.add_unique_non_send_sync(RectPrimitive(vert, index));
}
