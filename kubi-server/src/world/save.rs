use kubi_shared::data::{io_thread::IOThreadManager, open_local_save_file};
use shipyard::{AllStoragesView, UniqueView};

use crate::config::ConfigTable;

pub fn init_save_file(storages: &AllStoragesView) -> Option<IOThreadManager> {
  let config = storages.borrow::<UniqueView<ConfigTable>>().unwrap();
  if let Some(file_path) = &config.world.file {
    log::info!("Initializing save file from {:?}", file_path);
    let save = open_local_save_file(&file_path).unwrap();
    Some(IOThreadManager::new(save))
  } else {
    log::warn!("No save file specified, world will not be saved");
    None
  }
}
