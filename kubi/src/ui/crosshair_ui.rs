use std::f32::consts::PI;

use glam::{uvec2, Vec2};
use hui::{
  draw::{ImageHandle, TextureFormat},
  element::{container::Container, image::Image, transformer::ElementTransformExt, UiElementExt},
  layout::Alignment,
  size
};
use shipyard::{AllStoragesViewMut, IntoIter, NonSendSync, Unique, UniqueView, UniqueViewMut, View};
use crate::{hui_integration::UiState, player::MainPlayer, rendering::WindowSize, settings::GameSettings, world::raycast::LookingAtBlock};

const CROSSHAIR_SIZE: usize = 9;
const CROSSHAIR: &[u8] = &[
  0x00, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0x00,
  0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
  0x00, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0x00,
];

#[derive(Unique)]
pub struct CrosshairImage(ImageHandle);

pub fn init_crosshair_image(storages: AllStoragesViewMut) {
  let mut ui = storages.borrow::<NonSendSync<UniqueViewMut<UiState>>>().unwrap();
  let image = ui.hui.add_image(TextureFormat::Grayscale, CROSSHAIR, CROSSHAIR_SIZE);
  storages.add_unique(CrosshairImage(image));
}

pub fn draw_crosshair(
  mut ui: NonSendSync<UniqueViewMut<UiState>>,
  crosshair: UniqueView<CrosshairImage>,
  size: UniqueView<WindowSize>,
  player: View<MainPlayer>,
  raycast: View<LookingAtBlock>,
  settings: UniqueView<GameSettings>,
) {
  let mut active = false;
  if settings.dynamic_crosshair {
    if let Some((_, raycast)) = (&player, &raycast).iter().next() {
      active = raycast.0.is_some();
    }
  } else {
    active = true;
  }

  Container::default()
    .with_size(size!(100%))
    .with_align(Alignment::Center)
    .with_children(|ui| {
      Image::new(crosshair.0)
        .with_color((1., 1., 1., if active { 0.5 } else { 0.3 }))
        .with_size(size!(18, 18))
        .transform()
        .scale(Vec2::splat(if active { 1. } else { 0.66 }))
        .rotate(if active { 0. } else { PI / 4. })
        .add_child(ui);
    })
    .add_root(&mut ui.hui, uvec2(size.0.x & !1, size.0.y & !1).as_vec2());
}
