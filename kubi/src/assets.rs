use shipyard::{UniqueView, Unique, AllStoragesView};
use kubi_shared::block::BlockTexture;
use crate::rendering::Renderer;

mod texture;

use texture::load_asset_texture_array;

pub trait AssetPaths {
  fn file_name(self) -> &'static str;
}

impl AssetPaths for BlockTexture {
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
      Self::Cobblestone   => "cobblestone.png",
      Self::Planks        => "planks.png",
      Self::WaterSolid    => "solid_water.png",
    }
  }
}

#[derive(Unique)]
#[repr(transparent)]
pub struct BlockTexturesAsset(pub wgpu::Texture);

pub fn load_prefabs(
  storages: AllStoragesView,
  renderer: UniqueView<Renderer>
) {
  log::info!("Loading textures...");
  storages.add_unique(BlockTexturesAsset(
    load_asset_texture_array::<BlockTexture>("blocks".into(), &renderer)
  ));
}
