use shipyard::{World, NonSendSync, UniqueView, Unique};
use strum::{EnumIter, IntoEnumIterator};
use glium::{texture::{SrgbTexture2dArray, RawImage2d}, backend::Facade, Program};
use crate::rendering::Rederer;

mod texture;
mod shaders;

use texture::load_texture2darray_prefab;
use shaders::include_shader_prefab;

pub trait AssetPaths {
  fn file_name(self) -> &'static str;
}

#[derive(Clone, Copy, Debug, EnumIter)]
#[repr(u8)]
pub enum BlockTextures {
  Stone         = 0,
  Dirt          = 1,
  GrassTop      = 2,
  GrassSide     = 3,
  Sand          = 4,
  Bedrock       = 5,
  Wood          = 6,
  WoodTop       = 7,
  Leaf          = 8,
  Torch         = 9,
  TallGrass     = 10,
  Snow          = 11,
  GrassSideSnow = 12,
}
impl AssetPaths for BlockTextures {
  fn file_name(self) -> &'static str {
    match self {
      Self::Stone         => "stone.png",
      Self::Dirt          => "dirt.png",
      Self::GrassTop      => "grass_top.png",
      Self::GrassSide     => "grass_side.png",
      Self::Sand          => "sand.png",
      Self::Bedrock       => "bedrock.png",
      Self::Wood          => "wood.png",
      Self::WoodTop       => "wood_top.png",
      Self::Leaf          => "leaf.png",
      Self::Torch         => "torch.png",
      Self::TallGrass     => "tall_grass.png",
      Self::Snow          => "snow.png",
      Self::GrassSideSnow => "grass_side_snow.png",
    }
  }
}



#[derive(Unique)]
pub struct BlockTexturesPrefab(SrgbTexture2dArray);

#[derive(Unique)]
pub struct ChunkShaderPrefab(Program);

pub fn load_prefabs(world: &World) {
  let renderer = world.borrow::<NonSendSync<UniqueView<Rederer>>>().unwrap();
  world.add_unique_non_send_sync(BlockTexturesPrefab(
    load_texture2darray_prefab::<BlockTextures, _>(
      "./assets/blocks/".into(), 
      &renderer.display
    )
  ));
  world.add_unique_non_send_sync(ChunkShaderPrefab(
    include_shader_prefab!(
      "../shaders/world.vert",
      "../shaders/world.frag",
      &renderer.display
    )
  ));
}
