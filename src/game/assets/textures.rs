use std::{fs, io, path::PathBuf, sync::atomic::AtomicU16};
use rayon::prelude::*;
use glium::texture::{RawImage2d, SrgbTexture2d, SrgbTexture2dArray};

//This code is terrible and has a alot of duplication

fn load_png(file_path: &str, display: &glium::Display) -> SrgbTexture2d {
  log::info!("loading texture {}", file_path);

  //Load file
  let data = fs::read(file_path)
    .unwrap_or_else(|_| panic!("Failed to load texture: {}", file_path));
  
  //decode image data
  let image_data = image::load(
    io::Cursor::new(&data),
    image::ImageFormat::Png
  ).unwrap().to_rgba8();

  //Create raw glium image
  let image_dimensions = image_data.dimensions();
  let raw_image = RawImage2d::from_raw_rgba_reversed(
    &image_data.into_raw(), 
    image_dimensions
  );
  
  //Create texture
  SrgbTexture2d::new(display, raw_image).unwrap()
}

fn load_png_array(file_paths: &[PathBuf], display: &glium::Display) -> SrgbTexture2dArray {
  let counter = AtomicU16::new(0);
  let raw_images: Vec<RawImage2d<u8>> = file_paths.par_iter().enumerate().map(|(_, file_path)| {
    let counter = counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    log::info!("loading texture {}/{}: {}", counter, file_paths.len(), file_path.to_str().unwrap());

    //Load file
    let data = fs::read(file_path).expect("Failed to load texture");

    //decode image data
    let image_data = image::load(
      io::Cursor::new(&data),
      image::ImageFormat::Png
    ).unwrap().to_rgba8();

    //Create raw glium image
    let image_dimensions = image_data.dimensions();
    let raw_image = RawImage2d::from_raw_rgba_reversed(
      &image_data.into_raw(), 
      image_dimensions
    );

    raw_image
  }).collect();
  SrgbTexture2dArray::new(display, raw_images).unwrap()
}

pub struct Textures {
  pub blocks: SrgbTexture2dArray
}
impl Textures {
  /// Load textures synchronously, one by one and upload them to the GPU
  pub fn load_sync(display: &glium::Display) -> Self {
    Self {
      blocks: load_png_array(&[
        "./assets/blocks/stone.png".into(),
        "./assets/blocks/dirt.png".into(),
        "./assets/blocks/grass.png".into(),
        "./assets/blocks/grass_side.png".into(),
        "./assets/blocks/sand.png".into(),
        "./assets/blocks/bedrock.png".into(),
        "./assets/blocks/tree.png".into(),
        "./assets/blocks/tree_top.png".into(),
        "./assets/blocks/leaf.png".into(),
        "./assets/blocks/torch.png".into(),
        "./assets/blocks/tall_grass.png".into(),
      ], display)
    }
  }
}
