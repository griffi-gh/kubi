use shipyard::{Unique, AllStoragesView, NonSendSync, UniqueView};
use wgpu::util::DeviceExt;
use crate::rendering::Renderer;
use super::PositionVertex2d;

#[derive(Unique)]
pub struct RectPrimitive {
  pub vert: wgpu::Buffer,
  pub index: wgpu::Buffer,
}

const RECT_VERTEX: &[PositionVertex2d] = &[
  PositionVertex2d { position: [0., 0.] },
  PositionVertex2d { position: [1., 0.] },
  PositionVertex2d { position: [0., 1.] },
  PositionVertex2d { position: [1., 1.] },
];
const RECT_INDEX: &[u16] = &[0, 1, 2, 1, 3, 2];

pub(super) fn init_rect_primitive(
  storages: AllStoragesView,
  renderer: UniqueView<Renderer>
) {
  let vert = renderer.device.create_buffer_init(
    &wgpu::util::BufferInitDescriptor {
      label: Some("RectPrimitiveVertexBuffer"),
      contents: bytemuck::cast_slice(RECT_VERTEX),
      usage: wgpu::BufferUsages::VERTEX,
    }
  );
  let index = renderer.device.create_buffer_init(
    &wgpu::util::BufferInitDescriptor {
      label: Some("RectPrimitiveIndexBuffer"),
      contents: bytemuck::cast_slice(RECT_INDEX),
      usage: wgpu::BufferUsages::INDEX,
    }
  );
  storages.add_unique(RectPrimitive { vert, index });
}
