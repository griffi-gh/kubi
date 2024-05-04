// use shipyard::{Unique, AllStoragesView, NonSendSync, UniqueView};
// use glium::{VertexBuffer, IndexBuffer, index::PrimitiveType};
// use crate::rendering::Renderer;
// use super::PositionOnlyVertex2d;

// #[derive(Unique)]
// pub struct STriPrimitive(pub VertexBuffer<PositionOnlyVertex2d>, pub IndexBuffer<u16>);

// const STRI_VERTEX: &[PositionOnlyVertex2d] = &[
//   PositionOnlyVertex2d { position: [-1., -1.] },
//   PositionOnlyVertex2d { position: [ 3., -1.] },
//   PositionOnlyVertex2d { position: [-1.,  3.] },
// ];
// const STRI_INDEX: &[u16] = &[0, 1, 2];

// pub(super) fn init_stri_primitive(
//   storages: AllStoragesView,
//   display: NonSendSync<UniqueView<Renderer>>
// ) {
//   let vert = VertexBuffer::immutable(
//     &display.display,
//     STRI_VERTEX
//   ).unwrap();
//   let index = IndexBuffer::immutable(
//     &display.display,
//     PrimitiveType::TrianglesList,
//     STRI_INDEX
//   ).unwrap();
//   storages.add_unique_non_send_sync(STriPrimitive(vert, index));
// }
