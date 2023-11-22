use glam::vec2;
use kubi_ui::{KubiUi, backend::glium::GliumUiRenderer, element::{progress_bar::ProgressBar, UiElement}, UiSize};
use shipyard::{AllStoragesView, Unique, UniqueView, NonSendSync, UniqueViewMut};
use crate::rendering::{Renderer, RenderTarget, WindowSize};

#[derive(Unique)]
pub struct UiState {
  pub ui: KubiUi,
  pub renderer: GliumUiRenderer,
}

pub fn kubi_ui_init(
  storages: AllStoragesView
) {
  let renderer = storages.borrow::<NonSendSync<UniqueView<Renderer>>>().unwrap();
  storages.add_unique_non_send_sync(UiState {
    ui: KubiUi::new(),
    renderer: GliumUiRenderer::new(&renderer.display)
  });
}

pub fn kubi_ui_begin(
  mut ui: NonSendSync<UniqueViewMut<UiState>>
) {
  ui.ui.begin();
  ui.ui.add(ProgressBar {
    size: (UiSize::Pixels(300.), UiSize::Auto),
    ..Default::default()
  }, vec2(999., 999.));
}

pub fn kubi_ui_end(
  mut ui: NonSendSync<UniqueViewMut<UiState>>
) {
  let ui: &mut UiState = &mut ui;
  let UiState { ui, renderer } = ui;
  ui.end();
  let (upload_needed, plan) = ui.draw_plan();
  if upload_needed {
    renderer.update(plan);
  }
}

pub fn kubi_ui_draw(
  ui: NonSendSync<UniqueView<UiState>>,
  mut target: NonSendSync<UniqueViewMut<RenderTarget>>,
  size: UniqueView<WindowSize>
) {
  ui.renderer.draw(&mut target.0, size.0.as_vec2());
}
