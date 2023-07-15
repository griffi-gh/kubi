use shipyard::{Unique, AllStoragesView, NonSendSync, UniqueView};
use crate::rendering::Renderer;
use super::PositionVertex2d;

#[derive(Unique)]
pub struct RectPrimitive(pub VertexBuffer<PositionVertex2d>, pub IndexBuffer<u16>);

const RECT_VERTEX: &[PositionVertex2d] = &[
  PositionVertex2d { position: [0., 0.] },
  PositionVertex2d { position: [1., 0.] },
  PositionVertex2d { position: [0., 1.] },
  PositionVertex2d { position: [1., 1.] },
];
const RECT_INDEX: &[u16] = &[0, 1, 2, 1, 3, 2];

pub(super) fn init_rect_primitive(
  storages: AllStoragesView,
  display: NonSendSync<UniqueView<Renderer>>
) {
  let vert = VertexBuffer::new(
    &display.display,
    RECT_VERTEX
  ).unwrap();
  let index = IndexBuffer::new(
    &display.display,
    PrimitiveType::TrianglesList, 
    RECT_INDEX
  ).unwrap();
  storages.add_unique_non_send_sync(RectPrimitive(vert, index));
}
