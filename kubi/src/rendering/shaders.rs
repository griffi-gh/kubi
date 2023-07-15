use shipyard::{Unique, NonSendSync, UniqueView, AllStoragesView};

use super::Renderer;

#[derive(Unique)]
pub struct Shaders {
  pub world: wgpu::ShaderModule
}

macro_rules! shaders {
  {$renderer: expr, $dir: literal, $($name: ident -> $path: literal),*} => {
    {
      use super::Renderer;
      let renderer: &Renderer = $renderer;
      $({
        let _is_string_literal: &str = $path;
        renderer.device.create_shader_module(wgpu::include_wgsl!(concat!($dir, "/", $path)));
      })*
    }
  };
}

pub fn compile_shaders(
  storages: AllStoragesView,
) {
  let renderer = &storages.borrow::<NonSendSync<UniqueView<Renderer>>>().unwrap();
  shaders! {
    renderer, "../../shaders",
    world -> "world.wgsl"
  };
}
