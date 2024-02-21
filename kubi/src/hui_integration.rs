use std::{io::{BufReader, prelude::*}, path::Path};
use hui::{text::FontHandle, UiInstance};
use hui_glium::GliumUiRenderer;
use shipyard::{AllStoragesView, Unique, UniqueView, NonSendSync, UniqueViewMut};
use crate::{filesystem::AssetManager, rendering::{RenderTarget, Renderer, WindowSize}};

#[derive(Unique)]
pub struct UiState {
  pub hui: UiInstance,
  pub renderer: GliumUiRenderer,
  //HACK: This is a temporary solution, i don't think fonts should be stored here
  pub fonts: Vec<FontHandle>,
}

pub fn kubi_ui_init(
  storages: AllStoragesView
) {
  let renderer = storages.borrow::<NonSendSync<UniqueView<Renderer>>>().unwrap();
  storages.add_unique_non_send_sync(UiState {
    hui: UiInstance::new(),
    renderer: GliumUiRenderer::new(&renderer.display),
    fonts: vec![],
  });
}

//TODO: Use prefab system for this
pub fn kubi_ui_load_assets(
  asset_manager: UniqueView<AssetManager>,
  mut ui: NonSendSync<UniqueViewMut<UiState>>
) {
  let asset_handle = asset_manager.open_asset(Path::new("fonts/Crisp.ttf")).unwrap();
  let mut font_data = vec![];
  BufReader::new(asset_handle).read_to_end(&mut font_data).unwrap();
  let font_handle = ui.hui.add_font_from_bytes(&font_data);
  ui.fonts.push(font_handle);
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
  let UiState { hui, renderer, .. } = ui;
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
