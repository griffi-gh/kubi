use shipyard::{View, IntoIter, NonSendSync, UniqueViewMut, UniqueView, AllStoragesView, Unique};
use glium::{
  Surface, 
  implement_vertex, 
  IndexBuffer, 
  index::PrimitiveType, 
  VertexBuffer, uniform, 
  DrawParameters, 
  BackfaceCullingMode, 
  Blend, Depth, DepthTest,
};
use crate::{
  world::raycast::LookingAtBlock, 
  camera::Camera, prefabs::SelBoxShaderPrefab
};
use super::{
  RenderTarget, 
  primitives::{CUBE_INDICES, CUBE_VERTICES}, Renderer
};

#[derive(Clone, Copy, Default)]
pub struct SelBoxVertex {
  pub position: [f32; 3],
}
implement_vertex!(SelBoxVertex, position);

const fn box_vertices() -> [SelBoxVertex; CUBE_VERTICES.len() / 3] {
  let mut arr = [SelBoxVertex { position: [0., 0., 0.] }; CUBE_VERTICES.len() / 3];
  let mut ptr = 0;
  loop {
    arr[ptr] = SelBoxVertex {
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
const BOX_VERTICES: &[SelBoxVertex] = &box_vertices();

#[derive(Unique)]
pub struct SelectionBoxBuffers(VertexBuffer<SelBoxVertex>, IndexBuffer<u16>);

//wip
pub fn render_selection_box(
  lookat: View<LookingAtBlock>,
  camera: View<Camera>,
  mut target: NonSendSync<UniqueViewMut<RenderTarget>>, 
  display: NonSendSync<UniqueView<Renderer>>,
  program: NonSendSync<UniqueView<SelBoxShaderPrefab>>,
  buffers: NonSendSync<UniqueView<SelectionBoxBuffers>>,
) {
  let camera = camera.iter().next().unwrap();
  let Some(lookat) = lookat.iter().next() else { return };
  let Some(lookat) = lookat.0 else { return };

  //Darken block
  target.0.draw(
    &buffers.0,
    &buffers.1,
    &program.0,
    &uniform! {
      u_color: [0., 0., 0., 0.5_f32],
      u_position: lookat.block_position.as_vec3().to_array(),
      perspective: camera.perspective_matrix.to_cols_array_2d(),
      view: camera.view_matrix.to_cols_array_2d(),
    },
    &DrawParameters {
      backface_culling: BackfaceCullingMode::CullClockwise,
      blend: Blend::alpha_blending(),
      depth: Depth {
        test: DepthTest::IfLessOrEqual, //this may be unreliable!
        ..Default::default()
      },
      ..Default::default()
    }
  ).unwrap();
}

pub fn init_selection_box_buffers(
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
  storages.add_unique_non_send_sync(SelectionBoxBuffers(vert, index));
}
