use shipyard::{UniqueView, UniqueViewMut, Workload, IntoWorkload, EntityId, Unique, AllStoragesViewMut, ViewMut, Get, SystemModificator, track};
use glium::glutin::event::VirtualKeyCode;
use glam::{Mat3, vec2};
use crate::{
  world::ChunkStorage, 
  state::{GameState, NextState, is_changing_state}, 
  transform::Transform2d,
  gui::{
    GuiComponent, 
    progressbar::ProgressbarComponent
  },
  rendering::{WindowSize, if_resized}, 
  input::RawKbmInputState, 
};

#[derive(Unique, Clone, Copy)]
struct ProgressbarId(EntityId);

fn spawn_loading_screen(
  mut storages: AllStoragesViewMut,
) {
  let size = *storages.borrow::<UniqueView<WindowSize>>().unwrap();
  let entity = storages.add_entity((
    GuiComponent,
    Transform2d(Mat3::from_scale_angle_translation(
      vec2(size.0.x as f32, 16.), 
      0.,
      vec2(0., 0.)
    )),
    ProgressbarComponent {
      progress: 0.33
    },
  ));
  storages.add_unique(ProgressbarId(entity));
}

fn resize_progress_bar(
  size: UniqueView<WindowSize>,
  bar: UniqueView<ProgressbarId>,
  mut transforms: ViewMut<Transform2d, { track::All }>
) {
  let mut trans = (&mut transforms).get(bar.0).unwrap();
  trans.0.x_axis.x = size.0.x as f32;
}

fn update_progress_bar_progress (
  world: UniqueView<ChunkStorage>,
  mut bar: ViewMut<ProgressbarComponent>,
  eid: UniqueView<ProgressbarId>,
) {
  let mut bar = (&mut bar).get(eid.0).unwrap();
  let loaded = world.chunks.iter().fold(0, |acc, (&_, chunk)| {
    acc + chunk.desired_state.matches_current(chunk.current_state) as usize
  });
  let total = world.chunks.len();
  let progress = loaded as f32 / total as f32;
  bar.progress = progress;
}

fn switch_to_ingame_if_loaded(
  world: UniqueView<ChunkStorage>,
  mut state: UniqueViewMut<NextState>
) {
  if world.chunks.is_empty() {
    return
  }
  if world.chunks.iter().all(|(_, chunk)| {
    chunk.desired_state.matches_current(chunk.current_state)
  }) {
    log::info!("Finished loading chunks");
    state.0 = Some(GameState::InGame);
  }
}

fn override_loading(
  kbm_state: UniqueView<RawKbmInputState>,
  mut state: UniqueViewMut<NextState>
) {
  if kbm_state.keyboard_state.contains(&VirtualKeyCode::F) {
    state.0 = Some(GameState::InGame);
  }
}

fn despawn_loading_screen_if_switching_state(
  mut storages: AllStoragesViewMut,
) {
  let state = storages.borrow::<UniqueView<NextState>>().unwrap().0.unwrap();
  if state != GameState::LoadingWorld {
    let progress_bar = storages.borrow::<UniqueView<ProgressbarId>>().unwrap().0;
    storages.delete_entity(progress_bar);
    storages.remove_unique::<ProgressbarId>().unwrap();
  }
}

pub fn update_loading_screen() -> Workload {
  (
    spawn_loading_screen.run_if_missing_unique::<ProgressbarId>(),
    resize_progress_bar.run_if(if_resized),
    update_progress_bar_progress,
    override_loading,
    switch_to_ingame_if_loaded,
    despawn_loading_screen_if_switching_state.run_if(is_changing_state),
  ).into_sequential_workload()
}
