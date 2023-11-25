use kubi_ui::{KubiUi, backend::glium::GliumUiRenderer};
use shipyard::{AllStoragesView, Unique, UniqueView, NonSendSync, UniqueViewMut};
use crate::rendering::{Renderer, RenderTarget, WindowSize};

#[derive(Unique)]
pub struct UiState {
  pub kui: KubiUi,
  pub renderer: GliumUiRenderer,
}

pub fn kubi_ui_init(
  storages: AllStoragesView
) {
  let renderer = storages.borrow::<NonSendSync<UniqueView<Renderer>>>().unwrap();
  storages.add_unique_non_send_sync(UiState {
    kui: KubiUi::new(),
    renderer: GliumUiRenderer::new(&renderer.display)
  });
}

pub fn kubi_ui_begin(
  mut ui: NonSendSync<UniqueViewMut<UiState>>
) {
  ui.kui.begin();
}

pub fn kubi_ui_end(
  mut ui: NonSendSync<UniqueViewMut<UiState>>
) {
  let ui: &mut UiState = &mut ui;
  let UiState { kui, renderer } = ui;
  kui.end();
  let (upload_needed, plan) = kui.draw_plan();
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
