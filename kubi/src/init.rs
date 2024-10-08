use shipyard::{AllStoragesView, UniqueViewMut};
use std::{env, net::SocketAddr, path::Path};
use crate::{
  networking::{GameType, ServerAddress},
  state::{GameState, NextState}
};
use kubi_shared::data::{io_thread::IOThreadManager, open_local_save_file};

pub fn initialize_from_args(
  all_storages: AllStoragesView,
) {
  // If an address is provided, we're in multiplayer mode (the first argument is the address)
  // Otherwise, we're in singleplayer mode and working with local stuff
  let args: Vec<String> = env::args().collect();
  if args.len() > 1 {
    // Parse the address and switch the state to connecting
    let address = args[1].parse::<SocketAddr>().expect("invalid address");
    all_storages.add_unique(GameType::Muliplayer);
    all_storages.add_unique(ServerAddress(address));
    all_storages.borrow::<UniqueViewMut<NextState>>().unwrap().0 = Some(GameState::Connecting);
  } else {
    // Open the local save file
    let save_file = open_local_save_file(Path::new("./world.kubi")).expect("failed to open save file");
    all_storages.add_unique(IOThreadManager::new(save_file));
    // Switch the state and kick off the world loading
    all_storages.add_unique(GameType::Singleplayer);
    all_storages.borrow::<UniqueViewMut<NextState>>().unwrap().0 = Some(GameState::LoadingWorld);
  }
}
