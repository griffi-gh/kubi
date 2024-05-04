// use strum::IntoEnumIterator;
// use rayon::prelude::*;
// use std::{path::PathBuf, io::BufReader};
// use crate::filesystem::AssetManager;
// use super::AssetPaths;

// pub fn load_texture2darray_prefab<
//   T: AssetPaths + IntoEnumIterator,
//   E: Facade
// >(
//   assman: &AssetManager,
//   directory: PathBuf,
//   facade: &E,
//   mipmaps: MipmapsOption,
// ) -> SrgbTexture2dArray {
//   log::info!("started loading {}", directory.as_os_str().to_str().unwrap());
//   //Load raw images
//   let tex_files: Vec<&'static str> = T::iter().map(|x| x.file_name()).collect();
//   let raw_images: Vec<RawImage2d<u8>> = tex_files.par_iter().map(|&file_name| {
//     log::info!("loading texture {}", file_name);
//     //Get path to the image and open the file
//     let reader = {
//       let path = directory.join(file_name);
//       BufReader::new(assman.open_asset(&path).expect("Failed to open texture file"))
//     };
//     //Parse image data
//     let (image_data, dimensions) = {
//       let image = image::load(
//         reader,
//         image::ImageFormat::Png
//       ).unwrap().to_rgba8();
//       let dimensions = image.dimensions();
//       (image.into_raw(), dimensions)
//     };
//     //Create a glium RawImage
//     RawImage2d::from_raw_rgba_reversed(
//       &image_data,
//       dimensions
//     )
//   }).collect();
//   log::info!("done loading texture files, uploading to the gpu");
//   //Upload images to the GPU
//   SrgbTexture2dArray::with_mipmaps(facade, raw_images, mipmaps)
//     .expect("Failed to upload texture array to GPU")
// }
