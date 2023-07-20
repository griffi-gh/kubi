use glam::UVec2;
use strum::IntoEnumIterator;
use rayon::prelude::*;
use std::{path::PathBuf, io::BufReader, sync::Mutex, num::NonZeroU32};
use crate::{filesystem::open_asset, rendering::Renderer};
use super::AssetPaths;

pub fn load_asset_texture_array<
  T: AssetPaths + IntoEnumIterator,
>(
  directory: PathBuf,
  renderer: &Renderer,
) -> wgpu::Texture {
  log::info!("started loading texture array from: \"{}\"", directory.as_os_str().to_str().unwrap());
  
  //Load raw images
  let (texture_data, texture_dimensions): (Vec<Vec<u8>>, UVec2) = {
    //Image dimensions
    //Mutex is required to ensure exact size, without extra temporary allocations
    let img_dim: Mutex<Option<(NonZeroU32, NonZeroU32)>> = Mutex::new(None);

    //Get image file names into a Vec (because par_iter can't be called directly on IntoEnumIterator::iter())
    let file_names: Vec<&'static str> = T::iter().map(|x| x.file_name()).collect();

    //Load data in parallel
    let raw_images: Vec<Vec<u8>> = file_names.par_iter().map(|&file_name| {
      log::info!("loading texture \"{file_name}\"...");

      //Get path to the image and open the file
      let reader = {
        let path = directory.join(file_name);
        BufReader::new(open_asset(&path).expect("Failed to open texture file"))
      };

      //Load and parse image data
      let image = image::load(
        reader,
        image::ImageFormat::Png
      ).unwrap().to_rgba8();

      //Get image dimensions
      let dimensions = image.dimensions();
      let dim_nonzero = (
        NonZeroU32::new(dimensions.0).expect("image dimensions must be non-zero"),
        NonZeroU32::new(dimensions.1).expect("image dimensions must be non-zero")
      );

      //Ensure same size
      if let Ok(mut img_dim) = img_dim.lock() {
        if let Some(current_size) = img_dim.replace(dim_nonzero) {
          assert!(dim_nonzero == current_size, "image dimensions do not match");
        }
      }

      image.into_raw()
    }).collect();
    
    //Lock for the final time and retrieve the dimensions
    let img_dim = img_dim.lock().unwrap()
      .expect("no images were loaded, cannot create an empty texture array");
    let img_dim_vec = UVec2::new(img_dim.0.get(), img_dim.1.get());

    (raw_images, img_dim_vec)
  };

  //Flatten the texture data
  let texture_data_flat = texture_data.concat();
  
  log::info!("done loading texture files, uploading to the gpu");

  let texture_extent = wgpu::Extent3d {
    width: texture_dimensions.x,
    height: texture_dimensions.y,
    depth_or_array_layers: texture_data.len() as u32,
  };

  //Create a wgpu texture
  let texture_handle = renderer.device.create_texture(
    &wgpu::TextureDescriptor {
      size: texture_extent,
      mip_level_count: 1,
      sample_count: 1,
      dimension: wgpu::TextureDimension::D2, ///XXX: is this supposed to be D2 for array tex.?
      format: wgpu::TextureFormat::Rgba8UnormSrgb,
      usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
      label: Some("diffuse_texture"),
      view_formats: &[],
    }
  );

  //Upload texture data
  renderer.queue.write_texture(
    wgpu::ImageCopyTexture {
      texture: &texture_handle,
      mip_level: 0,
      origin: wgpu::Origin3d::ZERO,
      aspect: wgpu::TextureAspect::All,
    },
    &texture_data_flat,
    wgpu::ImageDataLayout {
      offset: 0,
      bytes_per_row: Some(4 * texture_dimensions.x),
      rows_per_image: Some(texture_dimensions.y),
    },
    texture_extent,
  );

  texture_handle
}
