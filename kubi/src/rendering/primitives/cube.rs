use shipyard::{AllStoragesView, NonSendSync, UniqueView, Unique};
use wgpu::util::DeviceExt;
use crate::rendering::Renderer;
use super::PositionVertex;

#[derive(Unique)]
pub struct CubePrimitive {
  pub vert: wgpu::Buffer,
  pub vert_centered: wgpu::Buffer,
  pub idx: wgpu::Buffer
}

const CENTERED_CUBE_VERTICES: &[PositionVertex] = &[
  // front
  PositionVertex { position: [-0.5, -0.5, 0.5] },
  PositionVertex { position: [ 0.5, -0.5, 0.5] },
  PositionVertex { position: [ 0.5,  0.5, 0.5] },
  PositionVertex { position: [-0.5,  0.5, 0.5] },
  // back
  PositionVertex { position: [-0.5, -0.5, -0.5] },
  PositionVertex { position: [ 0.5, -0.5, -0.5] },
  PositionVertex { position: [ 0.5,  0.5, -0.5] },
  PositionVertex { position: [-0.5,  0.5, -0.5] },
];
const CUBE_VERTICES: &[PositionVertex] = &[
  // front
  PositionVertex { position: [0.0, 0.0, 1.0] },
  PositionVertex { position: [1.0, 0.0, 1.0] },
  PositionVertex { position: [1.0, 1.0, 1.0] },
  PositionVertex { position: [0.0, 1.0, 1.0] },
  // back
  PositionVertex { position: [0.0, 0.0, 0.0] },
  PositionVertex { position: [1.0, 0.0, 0.0] },
  PositionVertex { position: [1.0, 1.0, 0.0] },
  PositionVertex { position: [0.0, 1.0, 0.0] },
];
const CUBE_INDICES: &[u16] = &[
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

pub(super) fn init_cube_primitive(
  storages: AllStoragesView,
  renderer: NonSendSync<UniqueView<Renderer>>
) {
  storages.add_unique(
    CubePrimitive {
      vert: renderer.device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
          label: Some("CubePrimitiveVertexBuffer"),
          contents: bytemuck::cast_slice(CUBE_VERTICES),
          usage: wgpu::BufferUsages::VERTEX,
        }
      ),
      vert_centered: renderer.device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
          label: Some("CubePrimitiveCenteredVertexBuffer"),
          contents: bytemuck::cast_slice(CUBE_VERTICES),
          usage: wgpu::BufferUsages::VERTEX,
        }
      ),
      idx: renderer.device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
          label: Some("CubePrimitiveIndexBuffer"),
          contents: bytemuck::cast_slice(CUBE_VERTICES),
          usage: wgpu::BufferUsages::INDEX,
        }
      )
    }
  );
}
