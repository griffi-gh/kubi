use std::{fs, io};
use glium::texture::{RawImage2d, SrgbTexture2d};

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

pub struct Textures {
    pub block_atlas: SrgbTexture2d
}
impl Textures {
    /// Load textures synchronously, one by one and upload them to the GPU
    pub fn load_sync(display: &glium::Display) -> Self {
        Self {
            block_atlas: load_png("assets/spritesheet.png", display)
        }
    }
}
