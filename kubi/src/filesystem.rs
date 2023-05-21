use std::{fs::File, path::Path, io::{Read, Seek}};
use anyhow::Result;

pub trait ReadOnly: Read + Seek {}
impl<T: Read + Seek> ReadOnly for T {}

#[allow(unreachable_code)]
pub fn open_asset(path: &Path) -> Result<Box<dyn ReadOnly>> {
  #[cfg(target_os = "android")] {
    use anyhow::Context;
    use std::ffi::CString;
    
    let asset_manager = ndk_glue::native_activity().asset_manager();
    let path_cstr = CString::new(path.to_string_lossy().as_bytes())?;
    let handle = asset_manager.open(&path_cstr).context("Asset doesn't exist")?;
    return Ok(Box::new(handle));
  }
  let asset_path = Path::new("./assets/").join(path);
  return Ok(Box::new(File::open(asset_path)?))
}
