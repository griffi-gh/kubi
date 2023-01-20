use shipyard::{World, NonSendSync, UniqueView, Unique};
use strum::{EnumIter, IntoEnumIterator};
use rayon::prelude::*;
use glium::{texture::{SrgbTexture2dArray, RawImage2d}, backend::Facade};
use std::{fs::File, path::PathBuf, io::BufReader};
use crate::rendering::Rederer;

trait AssetPaths {
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

fn load_texture2darray_prefab<T: AssetPaths + IntoEnumIterator, E: Facade>(directory: PathBuf, facade: &E) -> SrgbTexture2dArray {
  //Load raw images
  let tex_files: Vec<&'static str> = T::iter().map(|x| x.file_name()).collect();
  let raw_images: Vec<RawImage2d<u8>> = tex_files.par_iter().map(|&file_name| {
    log::info!("loading texture {}", file_name);
    //Get path to the image and open the file
    let reader = {
      let path = directory.join(file_name);
      BufReader::new(File::open(path).expect("Failed to open texture file"))
    };
    //Parse image data
    let (image_data, dimensions) = {
      let image =image::load(
        reader,
        image::ImageFormat::Png
      ).unwrap().to_rgba8();
      let dimensions = image.dimensions();
      (image.into_raw(), dimensions)
    };
    //Create a glium RawImage
    RawImage2d::from_raw_rgba_reversed(
      &image_data, 
      dimensions
    )
  }).collect();
  log::info!("done loading texture files, uploading to the gpu");
  //Upload images to the GPU
  SrgbTexture2dArray::new(facade, raw_images)
    .expect("Failed to upload texture array to GPU")
}

#[derive(Unique)]
pub struct BlockTexturesPrefab(SrgbTexture2dArray);

pub fn load_prefabs(world: &World) {
  let renderer = world.borrow::<NonSendSync<UniqueView<Rederer>>>().unwrap();
  world.add_unique_non_send_sync(BlockTexturesPrefab(
    load_texture2darray_prefab::<BlockTextures, _>("./assets/blocks/".into(), &renderer.display)
  ));
}
