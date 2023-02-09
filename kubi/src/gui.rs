use shipyard::{Component, Unique, Workload, IntoWorkload, AllStoragesView, AllStoragesViewMut};
use glam::{Vec2, Vec4, vec2};
use crate::color::color_hex;

pub mod text_widget;
pub mod progressbar;

use progressbar::{render_progressbars, ProgressbarComponent};

//TODO compute gui scale on window resize
#[derive(Unique, Clone, Copy, Debug)]
pub struct GuiViewScale(pub Vec2);

#[derive(Component, Clone, Copy, Debug, Default)]
pub struct GuiComponent;

#[derive(Component, Clone, Copy, Debug)]
#[track(All)]
pub struct GuiTransform {
  pub position: Vec2,
  pub scale: Vec2,
}
impl Default for GuiTransform {
  fn default() -> Self {
    Self {
      position: Vec2::ZERO,
      scale: Vec2::ONE,
    }
  }
}

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
  storages.add_unique(GuiViewScale(Vec2::ONE));
}

pub fn gui_testing(
  mut storages: AllStoragesViewMut,
) {
  storages.add_entity((
    GuiComponent,
    GuiTransform {
      position: Vec2::ZERO,
      scale: vec2(1.0, 0.05),
    },
    ProgressbarComponent {
      progress: 0.5
    },
    PrimaryColor::default(),
    SecondaryColor::default(),
  ));
}
