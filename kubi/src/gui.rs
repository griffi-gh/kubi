use shipyard::{Component, Unique, Workload, IntoWorkload, AllStoragesView, AllStoragesViewMut};
use glam::{Vec2, Vec4, Mat3, vec2, Mat4};
use crate::{color::color_hex, transform::Transform2d};

pub mod text_widget;
pub mod progressbar;

use progressbar::{render_progressbars, ProgressbarComponent};

//TODO compute gui scale on window resize
#[derive(Unique, Clone, Copy, Debug)]
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

pub fn render_gui() -> Workload {
  (
    render_progressbars
  ).into_workload()
}

pub fn init_gui(
  storages: AllStoragesView,
) {
  storages.add_unique(GuiView(Mat4::orthographic_rh_gl(0.0, 1.0, 1.0, 0.0, 0.0, 1.0)));
}

pub fn gui_testing(
  mut storages: AllStoragesViewMut,
) {
  storages.add_entity((
    GuiComponent,
    Transform2d(Mat3::from_scale_angle_translation(
      vec2(0.25, 0.05), 
      0.,
      vec2(0.5, 0.25)
    )),
    ProgressbarComponent {
      progress: 0.5
    },
    PrimaryColor::default(),
    SecondaryColor::default(),
  ));
}
