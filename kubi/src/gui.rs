use shipyard::{Component, Unique, Workload, IntoWorkload, AllStoragesView, View, UniqueViewMut, IntoIter};
use glam::{Vec4, Mat4};
use crate::{color::color_hex, events::WindowResizedEvent};

pub mod progressbar;

use progressbar::render_progressbars;

//TODO compute gui scale on window resize
#[derive(Unique, Clone, Copy, Debug, Default)]
pub struct GuiView(pub Mat4);

#[derive(Component, Clone, Copy, Debug, Default)]
pub struct GuiComponent;

#[derive(Component, Clone, Copy, Debug)]
pub struct PrimaryColor(pub Vec4);
impl Default for PrimaryColor {
  fn default() -> Self {
    Self(color_hex(0x156cddff))
  }
}

#[derive(Component, Clone, Copy, Debug)]
pub struct SecondaryColor(pub Vec4);
impl Default for SecondaryColor {
  fn default() -> Self {
    Self(color_hex(0xc9d5e4ff))
  }
}

fn update_gui_view(
  mut view: UniqueViewMut<GuiView>,
  resize: View<WindowResizedEvent>,
) {
  let Some(&size) = resize.iter().next() else {
    return
  };
  let [w, h] = size.0.to_array();
  view.0 = Mat4::orthographic_rh_gl(0.0, w as f32, h as f32, 0.0, -1.0, 1.0);
}

#[deprecated="will be replaced by an immediate-mode ui soon, currently a no-op"]
#[allow(deprecated)]
pub fn init_gui(
  storages: AllStoragesView
) {
  storages.add_unique(GuiView::default());
}

#[deprecated="will be replaced by an immediate-mode ui soon, currently a no-op"]
#[allow(deprecated)]
pub fn update_gui() -> Workload {
  (
    update_gui_view
  ).into_sequential_workload()
}

#[deprecated="will be replaced by an immediate-mode ui soon, currently a no-op"]
#[allow(deprecated)]
pub fn render_gui() -> Workload {
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
