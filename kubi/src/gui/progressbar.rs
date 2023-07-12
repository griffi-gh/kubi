use shipyard::{UniqueView, UniqueViewMut, NonSendSync, View, Component, IntoIter, IntoWithId, Get, track};
use crate::{
  prefabs::ProgressbarShaderPrefab,
  rendering::{
    RenderTarget,
    primitives::rect::RectPrimitive
  },
  transform::Transform2d,
};
use super::{GuiComponent, PrimaryColor, SecondaryColor, GuiView};

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct ProgressbarComponent {
  pub progress: f32
}

pub fn render_progressbars(
  mut target: NonSendSync<UniqueViewMut<RenderTarget>>,
  rect: NonSendSync<UniqueView<RectPrimitive>>,
  program: NonSendSync<UniqueView<ProgressbarShaderPrefab>>,
  view: UniqueView<GuiView>,
  components: View<GuiComponent>,
  transforms: View<Transform2d, track::All>,
  progressbars: View<ProgressbarComponent>,
  primary: View<PrimaryColor>,
  secondary: View<SecondaryColor>,
) {
  for (eid, (_, transform, progress)) in (&components, &transforms, &progressbars).iter().with_id() {
    let primary_color = primary.get(eid).copied().unwrap_or_default();
    let secondary_color = secondary.get(eid).copied().unwrap_or_default();
    // target.0.draw(
    //   &rect.0,
    //   &rect.1,
    //   &program.0,
    //   &uniform! {
    //     transform: transform.0.to_cols_array_2d(),
    //     ui_view: view.0.to_cols_array_2d(),
    //     progress: progress.progress,
    //     color: primary_color.0.to_array(),
    //     bg_color: secondary_color.0.to_array(),
    //   },
    //   &DrawParameters::default()
    // ).unwrap();
  }
}
