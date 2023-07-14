use shipyard::{NonSendSync, UniqueViewMut, UniqueView, View, IntoIter, IntoWithId};
use kubi_shared::{entity::Entity, transform::Transform};
use crate::{
  assets::ColoredShaderPrefab,
  camera::Camera,
  settings::GameSettings
};
use super::{
  RenderTarget, 
  primitives::cube::CenteredCubePrimitive
};

// TODO: entity models
pub fn render_entities(
  mut target: NonSendSync<UniqueViewMut<RenderTarget>>, 
  buffers: NonSendSync<UniqueView<CenteredCubePrimitive>>,
  program: NonSendSync<UniqueView<ColoredShaderPrefab>>,
  camera: View<Camera>,
  settings: UniqueView<GameSettings>,
  entities: View<Entity>,
  transform: View<Transform>,
) {
  #[cfg(fuck)] {
    let (camera_id, camera) = camera.iter().with_id().next().expect("No cameras in the scene");

    let draw_parameters = DrawParameters {
      depth: Depth {
        test: DepthTest::IfLess,
        write: true,
        ..Default::default()
      },
      multisampling: settings.msaa.is_some(),
      polygon_mode: PolygonMode::Fill,
      backface_culling: BackfaceCullingMode::CullClockwise,
      ..Default::default()
    };
    let view = camera.view_matrix.to_cols_array_2d();
    let perspective = camera.perspective_matrix.to_cols_array_2d();

    for (entity_id, (_, trans)) in (&entities, &transform).iter().with_id() {
      //skip rendering camera holder (as the entity would block the view)
      if entity_id == camera_id { continue }

      //render entity
      // target.0.draw(
      //   &buffers.0,
      //   &buffers.1,
      //   &program.0,
      //   &uniform! {
      //     color: [1.0, 1.0, 1.0, 1.0_f32],
      //     model: trans.0.to_cols_array_2d(),
      //     view: view,
      //     perspective: perspective,
      //   },
      //   &draw_parameters
      // ).unwrap();
    }
  }
}
