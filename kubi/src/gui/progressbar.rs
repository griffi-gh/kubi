use shipyard::{UniqueView, UniqueViewMut, NonSendSync, View, Component, IntoIter};
use glium::{Surface, uniform, DrawParameters};
use crate::{
  prefabs::ProgressbarShaderPrefab,
  rendering::{
    RenderTarget, 
    primitives::rect::RectPrimitive
  },
};
use super::{GuiComponent, GuiTransform, PrimaryColor, SecondaryColor, GuiViewScale};

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct ProgressbarComponent {
  pub progress: f32
}

pub fn render_progressbars(
  mut target: NonSendSync<UniqueViewMut<RenderTarget>>,
  rect: NonSendSync<UniqueView<RectPrimitive>>,
  program: NonSendSync<UniqueView<ProgressbarShaderPrefab>>,
  view: UniqueView<GuiViewScale>,
  components: View<GuiComponent>,
  transforms: View<GuiTransform>,
  progressbars: View<ProgressbarComponent>,
  primary: View<PrimaryColor>,
  secondary: View<SecondaryColor>,
) {
  for (_, transform, progress, pri, sec) in (&components, &transforms, &progressbars, &primary, &secondary).iter() {
    //TODO do this properly
    let pri = Some(pri).copied();
    let sec = Some(sec).copied();
    target.0.draw(
      &rect.0,
      &rect.1,
      &program.0,
      &uniform! {
        element_position: transform.position.to_array(),
        element_size: transform.scale.to_array(), 
        ui_view: view.0.to_array(),
        progress: progress.progress,
        color: pri.unwrap_or_default().0.to_array(),
        bg_color: sec.unwrap_or_default().0.to_array(),
      },
      &DrawParameters::default()
    ).unwrap();
  }
}
