use shipyard::{Component, Unique, Workload, IntoWorkload, AllStoragesView, View, UniqueViewMut, IntoIter};
use glam::{Vec4, Mat4};
use crate::{color::color_hex, events::WindowResizedEvent};

pub mod text_widget;
pub mod progressbar;

use progressbar::render_progressbars;

//TODO compute gui scale on window resize
#[derive(Unique, Clone, Copy, Debug, Default)]
pub struct LegacyGuiView(pub Mat4);

#[derive(Component, Clone, Copy, Debug, Default)]
pub struct LegacyGuiComponent;

#[derive(Component, Clone, Copy, Debug)]
pub struct LegacyPrimaryColor(pub Vec4);
impl Default for LegacyPrimaryColor {
  fn default() -> Self {
    Self(color_hex(0x156cddff))
  }
}

#[derive(Component, Clone, Copy, Debug)]
pub struct LegacySecondaryColor(pub Vec4);
impl Default for LegacySecondaryColor {
  fn default() -> Self {
    Self(color_hex(0xc9d5e4ff))
  }
}

fn update_legacy_gui_view(
  mut view: UniqueViewMut<LegacyGuiView>,
  resize: View<WindowResizedEvent>,
) {
  let Some(&size) = resize.iter().next() else {
    return
  };
  let [w, h] = size.0.to_array();
  view.0 = Mat4::orthographic_rh_gl(0.0, w as f32, h as f32, 0.0, -1.0, 1.0);
}

pub fn legacy_ui_init(
  storages: AllStoragesView
) {
  storages.add_unique(LegacyGuiView::default());
}

pub fn legacy_ui_update() -> Workload {
  (
    update_legacy_gui_view
  ).into_sequential_workload()
}

pub fn legacy_ui_render() -> Workload {
  (
    render_progressbars
  ).into_sequential_workload()
}

// pub fn gui_testing(
//   mut storages: AllStoragesViewMut,
// ) {
//   storages.add_entity((
//     GuiComponent,
//     Transform2d(Mat3::from_scale_angle_translation(
//       vec2(1920., 16.), 
//       0.,
//       vec2(0., 0.)
//     )),
//     ProgressbarComponent {
//       progress: 0.33
//     },
//     PrimaryColor::default(),
//     SecondaryColor::default(),
//   ));
// }
