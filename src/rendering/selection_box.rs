use shipyard::{View, IntoIter, NonSendSync, UniqueViewMut, UniqueView};
use glium::{
  Surface, 
  implement_vertex, 
  IndexBuffer, 
  index::PrimitiveType, 
  VertexBuffer, uniform, 
  DrawParameters, 
  BackfaceCullingMode, 
  Blend, BlendingFunction, 
  LinearBlendingFactor, 
  Depth, DepthTest,
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

//wip
pub fn render_selection_box(
  lookat: View<LookingAtBlock>,
  camera: View<Camera>,
  mut target: NonSendSync<UniqueViewMut<RenderTarget>>, 
  display: NonSendSync<UniqueView<Renderer>>,
  program: NonSendSync<UniqueView<SelBoxShaderPrefab>>,
) {
  let camera = camera.iter().next().unwrap();
  let Some(lookat) = lookat.iter().next() else { return };
  let Some(lookat) = lookat.0 else { return };

  //this may be slow but the amount of vertices is very low
  let vert = VertexBuffer::new(
    &display.display,
    BOX_VERTICES
  ).unwrap();
  let index = IndexBuffer::new(
    &display.display,
    PrimitiveType::TrianglesList, 
    CUBE_INDICES
  ).unwrap();

  target.0.draw(
    &vert,
    &index,
    &program.0,
    &uniform! {
      color: [0., 0., 0., 1.],
      u_position: lookat.block_position.as_vec3().to_array(),
      perspective: camera.perspective_matrix.to_cols_array_2d(),
      view: camera.view_matrix.to_cols_array_2d(),
    },
    &DrawParameters {
      backface_culling: BackfaceCullingMode::CullClockwise,
      blend: Blend {
        //for some reason only constant alpha works???
        color: BlendingFunction::Addition {
          source: LinearBlendingFactor::ConstantAlpha,
          destination: LinearBlendingFactor::OneMinusConstantAlpha,
        },
        alpha: BlendingFunction::Addition {
          source: LinearBlendingFactor::ConstantAlpha,
          destination: LinearBlendingFactor::OneMinusConstantAlpha
        },
        constant_value: (0.0, 0.0, 0.0, 0.5)
      },
      depth: Depth {
        test: DepthTest::IfLessOrEqual, //this may be unreliable!
        ..Default::default()
      },
      ..Default::default()
    }
  ).unwrap();
}
