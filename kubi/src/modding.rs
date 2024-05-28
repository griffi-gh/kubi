use std::{cell::RefCell, path::Path};
use glam::ivec3;
use kubi_shared::block::Block;
use shipyard::{AllStoragesView, IntoWorkload, NonSendSync, Unique, UniqueViewMut, Workload};
use kubi_mod_rt::{ModdingRuntime, ContextImpl};
use crate::world::{chunk::CurrentChunkState, ChunkStorage};

struct Context<'a> {
  cs: Option<&'a mut ChunkStorage>,
}

impl ContextImpl for Context<'_> {
  fn block(&self, x: i32, y: i32, z: i32) -> Option<u8> {
    self.cs.as_ref()?
      .get_block(ivec3(x, y, z))
      .map(|b| b as u8)
  }

  fn set_block(&mut self, x: i32, y: i32, z: i32, v: u8) -> bool {
    //TODO should this enqueue instead of setting directly?
    let Some(cs) = self.cs.as_mut() else { return false };
    let block = cs.get_block_mut(ivec3(x, y, z));
    if let Some(block) = block {
      //TODO handle the error here
      *block = Block::try_from(v).unwrap();
      true
    } else {
      false
    }
  }

  fn chunk_loaded(&self, x: i32, y: i32, z: i32) -> bool {
    self.cs.as_ref().unwrap().chunks
      .get(&ivec3(x, y, z))
      .map(|c| c.current_state >= CurrentChunkState::Loaded)
      .unwrap_or(false)
  }
}

#[derive(Unique)]
pub struct ModState {
  pub rt: ModdingRuntime,
}

pub fn init_modrt(storages: AllStoragesView) {
  storages.add_unique_non_send_sync(ModState {
    rt: ModdingRuntime::init(),
  });
}

pub fn load_mods(
  mut state: NonSendSync<UniqueViewMut<ModState>>,
) {
  log::info!("Loading mods");
  let path = Path::new("./mods");
  if path.exists() {
    state.rt.load_mod_dir(path).unwrap();
    log::info!("mods loaded: {:?}", state.rt.mods().len());
  } else {
    log::info!("no mod directory found, skipping");
  }
}

pub fn run_mod_init_stage(
  mut state: NonSendSync<UniqueViewMut<ModState>>,
  mut cs: Option<UniqueViewMut<ChunkStorage>>,
) {
  log::info!("running mod init stage");
  state.rt.run_init(&RefCell::new(Context {
    cs: cs.as_deref_mut(),
  }));
}

pub fn init_modding() -> Workload {
  (
    init_modrt,
    load_mods,
    run_mod_init_stage,
  ).into_workload()
}
