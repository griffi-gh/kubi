use std::{cell::RefCell, collections::HashMap, io::Read, path::{Path, PathBuf}};
use kubi_shared::networking::messages::PROTOCOL_ID as NET_PROTOCOL_ID;
use mlua::prelude::*;
use anyhow::Result;
use serde::Deserialize;

pub const LUA_GLOBAL_KEY: &str = "kubi";
pub const MOD_RUNTIME_VERSION: i32 = -1;

pub enum ExternalEvent {
  //Both server and client
  Tick { dt: f32 },
  BlockEvent { x: i32, y: i32, z: i32, block: u8 },
  ChunkTransitioned { x: i32, y: i32, z: i32, state: u8 },
  //Client only
  LocalBlockEvent { x: i32, y: i32, z: i32, block: u8 },
  LocalPlayerMoved { x: f32, y: f32, z: f32 },
}

pub trait ContextImpl {
  fn block(&self, x: i32, y: i32, z: i32) -> Option<u8>;
  fn set_block(&mut self, x: i32, y: i32, z: i32, v: u8) -> bool;
  fn chunk_loaded(&self, x: i32, y: i32, z: i32) -> bool;
}

struct ContextLuaUserData<'a, T: ContextImpl + 'a>(&'a RefCell<T>);

impl<'a, T: ContextImpl> LuaUserData for ContextLuaUserData<'a, T> {
  fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
    fields.add_field("runtime_version", MOD_RUNTIME_VERSION);
    fields.add_field("netcode_version", NET_PROTOCOL_ID);
  }

  fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
    // get_block(x: number, y: number, z: number) -> number?
    // Returns the block at the given position, or nil if the chunk is not loaded
    methods.add_method("get_block", |_lua, this, (x, y, z): (i32, i32, i32)| {
      Ok(this.0.borrow().block(x, y, z))
    });

    // set_block(x: number, y: number, z: number, v: number)
    // Sets the block at the given position, or errors if the chunk is not loaded
    methods.add_method("set_block", |_lua, this, (x, y, z, v): (i32, i32, i32, u8)| {
      this.0.borrow_mut()
        .set_block(x, y, z, v)
        .then_some(())
        .ok_or(LuaError::external("attempt to modify unloaded chunk"))
    });

    //returns true if the chunk is loaded, false otherwise
    //TODO: return full state (e.g. loaded/rendered/etc)
    methods.add_method("is_chunk_loaded", |_lua, this, (x, y, z): (i32, i32, i32)| {
      Ok(this.0.borrow().chunk_loaded(x, y, z))
    });
  }
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct ModMetadata {
  pub id: String,
  pub name: String,
  pub version: String,
  pub author: String,
  pub description: String,
  pub entry: PathBuf,
}

pub struct ModLuaState {
  //TODO
}

pub struct ModInstance {
  pub meta: ModMetadata,
  pub unpackdata: HashMap<PathBuf, Box<[u8]>>,
  pub state: Option<ModLuaState>,
}

pub struct ModdingRuntime {
  lua_state: mlua::Lua,
  mods: Vec<ModInstance>,
}

impl ModdingRuntime {
  pub fn init() -> Self {
    ModdingRuntime {
      lua_state: mlua::Lua::new(),
      mods: Vec::new(),
    }
  }

  /// Load a directory of mods into the runtime recursively
  /// This will only work on non-wasm targets etc
  pub fn load_mod_dir(&mut self, mod_dir: &Path) -> Result<()> {
    for entry in std::fs::read_dir(mod_dir)? {
      let entry = entry?;
      let path = entry.path();
      if path.is_dir() {
        self.load_mod_dir(&path)?;
      } else {
        self.load_mod(&path)?;
      }
    }
    Ok(())
  }

  /// Load a mod into the runtime
  pub fn load_mod(&mut self, mod_path: &Path) -> Result<()> {
    let mod_file = std::fs::File::open(mod_path)?;
    let mut files = tar::Archive::new(mod_file);
    let mut meta = None;
    let mut unpackdata = HashMap::new();
    for file in files.entries()? {
      let mut file = file?;

      let mut file_data_buf = vec![];
      file.read_to_end(&mut file_data_buf)?;

      let path = file.path()?;
      if path == Path::new("manifest.json") {
        // meta = Some(simd_json::serde::from_reader(&file)?);
        let file_cursor = std::io::Cursor::new(&file_data_buf[..]);
        meta = Some(simd_json::serde::from_reader(file_cursor)?);
      }

      unpackdata.insert(path.to_path_buf(), file_data_buf.into_boxed_slice());
    }
    let meta = meta.ok_or_else(|| anyhow::anyhow!("No manifest.json found"))?;
    let mod_instance = ModInstance {
      meta,
      unpackdata,
      state: None,
    };
    self.mods.push(mod_instance);
    Ok(())
  }

  /// Initialize all loaded mods
  pub fn run_init(&mut self, ctx: &RefCell<impl ContextImpl>) {
    for instance in &mut self.mods {
      if instance.state.is_some() {
        continue
      }
      let entry_path = instance.meta.entry.as_path();
      //TODO handle error
      let entry_file = instance.unpackdata.get(entry_path)
        .expect("entry file not found");
      let source = std::str::from_utf8(entry_file).unwrap();
      self.lua_state.scope(|scope| {
        let ctx = ContextLuaUserData(ctx);
        let userdata = scope
          .create_nonstatic_userdata(ctx)?;
        self.lua_state
          .globals()
          .set(LUA_GLOBAL_KEY, userdata).unwrap();
        let chunk = self.lua_state.load(source);
        chunk.exec()
      }).unwrap();
    }
  }

  pub fn mods(&self) -> &[ModInstance] {
    &self.mods
  }

  pub fn shove_event(&self, event: ExternalEvent) {
    //TODO
  }
}
