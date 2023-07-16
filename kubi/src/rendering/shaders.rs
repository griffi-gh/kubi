use shipyard::{Unique, NonSendSync, UniqueView, AllStoragesView};

use super::Renderer;

#[derive(Unique)]
pub struct Shaders {
  pub world: wgpu::ShaderModule,
  pub colored: wgpu::ShaderModule
}

macro_rules! upload_shaders {
  {$container: ident [$renderer: expr, $dir: literal] { $($name: ident : $path: literal $(,)*),*  } } => {{
    //ensure types
    let _: &super::Renderer = $renderer;
    let _: &str = $dir;
    $( let _: &str = $path; )*
    $container {
      $($name: {
        let source = include_str!(concat!($dir, "/", $path)).into();
        let descriptor = wgpu::ShaderModuleDescriptor {
          label: Some(stringify!($name)),
          source: wgpu::ShaderSource::Wgsl(source),
        };
        $renderer.device.create_shader_module(descriptor)
      },)*
    }
  }};
}

pub fn compile_shaders(
  storages: AllStoragesView,
) {
  let renderer = &storages.borrow::<NonSendSync<UniqueView<Renderer>>>().unwrap();
  let shaders = upload_shaders! {
    Shaders [renderer, "../../shaders"] {
      world: "world.wgsl",
      colored: "colored.wgsl"
    }
  };
}
