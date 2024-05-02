use shipyard::{AllStoragesView, UniqueViewMut};
use std::{env, net::SocketAddr, fs::OpenOptions, path::Path};
use anyhow::Result;
use crate::{
  networking::{GameType, ServerAddress},
  state::{GameState, NextState}
};
use kubi_shared::data::WorldSaveFile;

fn open_local_save_file(path: &Path) -> Result<WorldSaveFile> {
  let mut save_file = WorldSaveFile::new({
    OpenOptions::new()
      .read(true)
      .write(true)
      .open("world.kbi")?
  });
  if save_file.file.metadata().unwrap().len() == 0 {
    save_file.initialize()?;
  } else {
    save_file.load_data()?;
  }
  Ok(save_file)
}

pub fn initialize_from_args(
  all_storages: AllStoragesView,
) {
  let args: Vec<String> = env::args().collect();
  if args.len() > 1 {
    let address = args[1].parse::<SocketAddr>().expect("invalid address");
    all_storages.add_unique(GameType::Muliplayer);
    all_storages.add_unique(ServerAddress(address));
    all_storages.borrow::<UniqueViewMut<NextState>>().unwrap().0 = Some(GameState::Connecting);
  } else {
    all_storages.add_unique(GameType::Singleplayer);
    all_storages.borrow::<UniqueViewMut<NextState>>().unwrap().0 = Some(GameState::LoadingWorld);
  }
}
