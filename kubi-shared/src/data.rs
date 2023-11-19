use std::{
  fs::File,
  mem::size_of,
  io::{Read, Seek, SeekFrom, Write},
  borrow::Cow
};
use serde::{Serialize, Deserialize};
use glam::IVec2;
use hashbrown::HashMap;
use anyhow::Result;
use crate::{block::Block, chunk::{CHUNK_SIZE, BlockDataRef}};

const SECTOR_SIZE: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE * size_of::<Block>();
const RESERVED_SIZE: usize = 1048576; //~1mb (16 sectors assuming 32x32x32 world of 1byte blocks)
const RESERVED_SECTOR_COUNT: usize = RESERVED_SIZE / SECTOR_SIZE;

//magic = "KUBI" + IDENTITY (4 bytes)
const HEADER_MAGIC_SIZE: usize = 8;
const HEADER_MAGIC_STR: [u8; 4] = *b"KUBI";
const HEADER_MAGIC_IDENTITY: u32 = 1;

// #[repr(transparent)]
// struct IVec2Hash(IVec2);
#[derive(Serialize, Deserialize)]
struct WorldSaveDataHeader {
  pub name: Cow<'static, str>,
  pub seed: u64,
  sector_count: u32,
  chunk_map: HashMap<IVec2, u32>,
}

impl Default for WorldSaveDataHeader {
  fn default() -> Self {
    Self {
      name: "World".into(),
      seed: 0,
      sector_count: RESERVED_SECTOR_COUNT as u32,
      chunk_map: HashMap::new()
    }
  }
}

struct WorldSaveFile {
  pub file: File,
  pub header: WorldSaveDataHeader,
}

impl WorldSaveFile {
  pub fn new(file: File) -> Self {
    WorldSaveFile {
      file,
      header: WorldSaveDataHeader::default()
    }
  }

  fn read_header(&mut self) -> Result<()> {
    self.file.rewind()?;

    let mut subheader = [0u8; HEADER_MAGIC_SIZE];
    self.file.read_exact(&mut subheader)?;
    if subheader[0..4] != HEADER_MAGIC_STR {
      return Err(anyhow::anyhow!("invalid file header"));
    }
    if subheader[4..8] != HEADER_MAGIC_IDENTITY.to_be_bytes() {
      return Err(anyhow::anyhow!("this save file cannot be loaded by this version of the game"));
    }

    let limit = (RESERVED_SIZE - HEADER_MAGIC_SIZE) as u64;
    self.header = bincode::deserialize_from((&self.file).take(limit))?;

    Ok(())
  }

  fn write_header(&mut self) -> Result<()> {
    self.file.rewind()?;
    self.file.write_all(&HEADER_MAGIC_STR)?;
    self.file.write_all(&HEADER_MAGIC_IDENTITY.to_be_bytes())?;
    //XXX: this can cause the header to destroy chunk data (if it's WAY too long)
    //     read has checks against this, but write doesn't
    //     1mb is pretty generous tho, so it's not a *big* deal
    bincode::serialize_into(&self.file, &self.header)?;
    Ok(())
  }

  fn allocate_sector(&mut self) -> u32 {
    let value = self.header.sector_count + 1;
    self.header.sector_count += 1;
    value
  }

  pub fn save_chunk(&mut self, position: IVec2, data: &BlockDataRef) -> Result<()> {
    let mut header_modified = false;
    let sector = self.header.chunk_map.get(&position).copied().unwrap_or_else(|| {
      header_modified = true;
      self.allocate_sector()
    });

    let offset = sector as u64 * SECTOR_SIZE as u64;
    //SAFETY: *nuzzles* t-t-twust me pwease OwO
    let data: &[u8; SECTOR_SIZE] = unsafe { std::mem::transmute(data) };

    self.file.seek(SeekFrom::Start(offset))?;
    self.file.write_all(data)?;

    if header_modified {
      self.write_header()?;
    }
    self.file.sync_data()?;
    Ok(())
  }
}
