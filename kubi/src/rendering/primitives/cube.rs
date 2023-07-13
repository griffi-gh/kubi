use shipyard::{AllStoragesView, NonSendSync, UniqueView, Unique};
use crate::rendering::Renderer;
use super::PositionOnlyVertex;

#[derive(Unique)]
pub struct CubePrimitive {
  pub vert: wgpu::Buffer,
  pub idx: wgpu::Buffer
}

#[derive(Unique)]
pub struct CenteredCubePrimitive {
  pub vert: wgpu::Buffer,
  pub idx: wgpu::Buffer
}

const CENTERED_CUBE_VERTICES: &[PositionOnlyVertex] = &[
  // front
  PositionOnlyVertex { position: [-0.5, -0.5, 0.5] },
  PositionOnlyVertex { position: [ 0.5, -0.5, 0.5] },
  PositionOnlyVertex { position: [ 0.5,  0.5, 0.5] },
  PositionOnlyVertex { position: [-0.5,  0.5, 0.5] },
  // back
  PositionOnlyVertex { position: [-0.5, -0.5, -0.5] },
  PositionOnlyVertex { position: [ 0.5, -0.5, -0.5] },
  PositionOnlyVertex { position: [ 0.5,  0.5, -0.5] },
  PositionOnlyVertex { position: [-0.5,  0.5, -0.5] },
];
const CUBE_VERTICES: &[PositionOnlyVertex] = &[
  // front
  PositionOnlyVertex { position: [0.0, 0.0, 1.0] },
  PositionOnlyVertex { position: [1.0, 0.0, 1.0] },
  PositionOnlyVertex { position: [1.0, 1.0, 1.0] },
  PositionOnlyVertex { position: [0.0, 1.0, 1.0] },
  // back
  PositionOnlyVertex { position: [0.0, 0.0, 0.0] },
  PositionOnlyVertex { position: [1.0, 0.0, 0.0] },
  PositionOnlyVertex { position: [1.0, 1.0, 0.0] },
  PositionOnlyVertex { position: [0.0, 1.0, 0.0] },
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
  display: NonSendSync<UniqueView<Renderer>>
) {
  {
    let vert = VertexBuffer::new(
      &display.display,
      CUBE_VERTICES
    ).unwrap();
    let index = IndexBuffer::new(
      &display.display,
      PrimitiveType::TrianglesList, 
      CUBE_INDICES
    ).unwrap();
    storages.add_unique_non_send_sync(CubePrimitive(vert, index));
  }
  {
    let vert = VertexBuffer::new(
      &display.display,
      CENTERED_CUBE_VERTICES
    ).unwrap();
    let index = IndexBuffer::new(
      &display.display,
      PrimitiveType::TrianglesList, 
      CUBE_INDICES
    ).unwrap();
    storages.add_unique_non_send_sync(CenteredCubePrimitive(vert, index));
  }
}
