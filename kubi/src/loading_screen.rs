use shipyard::{UniqueView, UniqueViewMut, Workload, IntoWorkload, WorkloadModificator, EntityId, Unique, AllStoragesViewMut, ViewMut, View, IntoIter, Get};
use glam::{Mat3, vec2};
use crate::{
  world::ChunkStorage, 
  state::{GameState, NextState}, 
  transform::Transform2d,
  gui::{
    GuiComponent, 
    progressbar::ProgressbarComponent
  }, rendering::{WindowSize, if_resized}, 
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
  mut transforms: ViewMut<Transform2d>
) {
  let mut trans = (&mut transforms).get(bar.0).unwrap();
  trans.0.x_axis.x = size.0.x as f32;
}

fn update_progress_bar_progress (
  world: UniqueView<ChunkStorage>,
  mut bar: ViewMut<ProgressbarComponent>,
  eid: UniqueView<ProgressbarId>,
) {
  let bar = (&mut bar).get(eid.0).unwrap();
  let loaded = world.chunks.iter().fold(0, |acc, (&_, chunk)| {
    acc + chunk.desired_state.matches(chunk.current_state) as usize
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
    chunk.desired_state.matches(chunk.current_state)
  }) {
    log::info!("Finished loadinf chunks");
    state.0 = Some(GameState::InGame);
  }
}

fn despawn_loading_screen_if_switching_state(
  mut storages: AllStoragesViewMut,
) {
  let next = *storages.borrow::<UniqueView<NextState>>().unwrap();
  if let Some(state) = next.0 {
    if state != GameState::LoadingWorld {
      let progress_bar = storages.borrow::<UniqueView<ProgressbarId>>().unwrap().0;
      storages.delete_entity(progress_bar);
      storages.remove_unique::<ProgressbarId>().unwrap();
    }
  }
}

pub fn update_loading_screen() -> Workload {
  (
    spawn_loading_screen.into_workload().run_if_missing_unique::<ProgressbarId>(),
    resize_progress_bar.into_workload().run_if(if_resized),
    update_progress_bar_progress,
    switch_to_ingame_if_loaded,
    despawn_loading_screen_if_switching_state,
  ).into_workload()
}
