use glam::UVec2;
use strum::IntoEnumIterator;
use rayon::prelude::*;
use wgpu::util::{DeviceExt, TextureDataOrder};
use std::{io::BufReader, path::PathBuf};
use crate::{filesystem::AssetManager, rendering::Renderer};
use super::AssetPaths;

pub fn load_texture2darray_prefab<T: AssetPaths + IntoEnumIterator>(
  renderer: &Renderer,
  assman: &AssetManager,
  directory: PathBuf,
) -> wgpu::Texture {
  log::info!("started loading {}", directory.as_os_str().to_str().unwrap());

  //Load raw images
  let tex_files: Vec<&'static str> = T::iter().map(|x| x.file_name()).collect();
  let raw_images: Vec<(Vec<u8>, UVec2)> = tex_files.par_iter().map(|&file_name| {
    log::info!("loading texture {}", file_name);

    //Get path to the image and open the file
    let reader = {
      let path = directory.join(file_name);
      BufReader::new(assman.open_asset(&path).expect("Failed to open texture file"))
    };

    //Parse image data
    let (image_data, dimensions) = {
      let image = image::load(
        reader,
        image::ImageFormat::Png
      ).unwrap().to_rgba8();
      let dimensions = image.dimensions();
      (image.into_raw(), dimensions)
    };
    (image_data, UVec2::from(dimensions))
  }).collect();

  assert!(!raw_images.is_empty(), "no images loaded");
  //TODO: check same size

  log::info!("done loading texture files, uploading to the gpu");

  let size = raw_images[0].1;
  let layers = raw_images.len() as u32;

  //Concat data into a single vec
  let mut data = Vec::with_capacity((size.x * size.y * layers * 4) as usize);
  for (layer_data, _) in raw_images {
    data.extend_from_slice(&layer_data);
  }

  //Upload images to the GPU
  let desc = &wgpu::TextureDescriptor {
    label: Some("block_diffuse_texture"),
    size: wgpu::Extent3d {
      width: size.x,
      height: size.y,
      depth_or_array_layers: layers,
    },
    dimension: wgpu::TextureDimension::D2,
    format: wgpu::TextureFormat::Rgba8UnormSrgb,
    usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
    mip_level_count: 1,
    sample_count: 1,
    view_formats: &[],
  };

  renderer.device().create_texture_with_data(
    renderer.queue(),
    desc,
    TextureDataOrder::MipMajor,
    &data
  )
}
