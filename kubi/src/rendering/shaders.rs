use shipyard::{Unique, NonSendSync, UniqueView, AllStoragesView};

use super::Renderer;

#[derive(Unique)]
pub struct Shaders {
  pub world: wgpu::ShaderModule
}

macro_rules! shaders {
  {($renderer: expr, $dir: literal), $($name: ident : $path: literal),*} => {
    {
      use super::Renderer;
      let renderer: &Renderer = $renderer;
      $(
        let $name = {
          let _is_string_literal: &str = $path;
          let shader_descriptor = wgpu::include_wgsl!(concat!($dir, "/", $path));
          renderer.device.create_shader_module(shader_descriptor)
        };
      )*
      Shaders {
        $($name,)*
      }
    }
  };
}

pub fn compile_shaders(
  storages: AllStoragesView,
) {
  let renderer = &storages.borrow::<NonSendSync<UniqueView<Renderer>>>().unwrap();
  shaders! {
    (renderer, "../../shaders"),
    world: "world.wgsl"
  };
}
