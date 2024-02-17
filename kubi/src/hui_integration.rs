use hui::UiInstance;
use hui_glium::GliumUiRenderer;
use shipyard::{AllStoragesView, Unique, UniqueView, NonSendSync, UniqueViewMut};
use crate::rendering::{Renderer, RenderTarget, WindowSize};

#[derive(Unique)]
pub struct UiState {
  pub hui: UiInstance,
  pub renderer: GliumUiRenderer,
}

pub fn kubi_ui_init(
  storages: AllStoragesView
) {
  let renderer = storages.borrow::<NonSendSync<UniqueView<Renderer>>>().unwrap();
  storages.add_unique_non_send_sync(UiState {
    hui: UiInstance::new(),
    renderer: GliumUiRenderer::new(&renderer.display)
  });
}

pub fn kubi_ui_begin(
  mut ui: NonSendSync<UniqueViewMut<UiState>>
) {
  ui.hui.begin();
}

pub fn kubi_ui_end(
  mut ui: NonSendSync<UniqueViewMut<UiState>>
) {
  let ui: &mut UiState = &mut ui;
  let UiState { hui, renderer } = ui;
  hui.end();
  renderer.update(hui);
}

pub fn kubi_ui_draw(
  ui: NonSendSync<UniqueView<UiState>>,
  mut target: NonSendSync<UniqueViewMut<RenderTarget>>,
  size: UniqueView<WindowSize>
) {
  ui.renderer.draw(&mut target.0, size.0.as_vec2());
}
