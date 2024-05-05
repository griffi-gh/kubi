use std::{io::{BufReader, Read}, path::Path};
use hui::text::FontHandle;
use shipyard::{AllStoragesView, NonSendSync, Unique, UniqueView, UniqueViewMut};
use kubi_shared::block::BlockTexture;
use crate::{filesystem::AssetManager, hui_integration::UiState, rendering::Renderer};

mod texture;
use texture::load_texture2darray_prefab;

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
      Self::Water         => "water.png",
    }
  }
}

#[derive(Unique)]
pub struct TexturePrefabs {
  pub block_diffuse_texture: wgpu::Texture,
  pub block_diffuse_bind_group: wgpu::BindGroup,
}

#[derive(Unique)]
#[repr(transparent)]
pub struct UiFontPrefab(pub FontHandle);

pub fn load_prefabs(
  storages: AllStoragesView,
  renderer: NonSendSync<UniqueView<Renderer>>,
  mut ui: NonSendSync<UniqueViewMut<UiState>>,
  assman: UniqueView<AssetManager>
) {
  log::info!("Loading textures...");
  let block_diffuse_texture = load_texture2darray_prefab::<BlockTexture>(
    &renderer,
    &assman,
    "blocks".into(),
  );

  log::info!("Creating bing groups");
  let block_diffuse_view = block_diffuse_texture.create_view(&wgpu::TextureViewDescriptor::default());
  let block_diffuse_sampler = renderer.device().create_sampler(&wgpu::SamplerDescriptor {
    address_mode_u: wgpu::AddressMode::ClampToEdge,
    address_mode_v: wgpu::AddressMode::ClampToEdge,
    address_mode_w: wgpu::AddressMode::ClampToEdge,
    mag_filter: wgpu::FilterMode::Nearest,
    min_filter: wgpu::FilterMode::Nearest, //TODO min_filter Linear, requires filtering sampler
    mipmap_filter: wgpu::FilterMode::Nearest,
    ..Default::default()
  });
  let block_diffuse_bind_group_layout = renderer.device()
    .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
      label: Some("block_diffuse_bind_group_layout"),
      entries: &[
        wgpu::BindGroupLayoutEntry {
          binding: 0,
          visibility: wgpu::ShaderStages::FRAGMENT,
          ty: wgpu::BindingType::Texture {
            sample_type: wgpu::TextureSampleType::Float { filterable: false },
            view_dimension: wgpu::TextureViewDimension::D2Array,
            multisampled: false,
          },
          count: None,
        },
        wgpu::BindGroupLayoutEntry {
          binding: 1,
          visibility: wgpu::ShaderStages::FRAGMENT,
          ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
          count: None,
        }
      ]
    });
  let block_diffuse_bind_group = renderer.device().create_bind_group(&wgpu::BindGroupDescriptor {
    label: Some("block_diffuse_bind_group"),
    layout: &block_diffuse_bind_group_layout,
    entries: &[
      wgpu::BindGroupEntry {
        binding: 0,
        resource: wgpu::BindingResource::TextureView(&block_diffuse_view),
      },
      wgpu::BindGroupEntry {
        binding: 1,
        resource: wgpu::BindingResource::Sampler(&block_diffuse_sampler),
      }
    ]
  });
  storages.add_unique_non_send_sync(TexturePrefabs {
    block_diffuse_texture,
    block_diffuse_bind_group,
  });

  log::info!("Loading the UI stuff...");
  {
    let asset_handle = assman.open_asset(Path::new("fonts/Crisp.ttf")).unwrap();
    let mut font_data = vec![];
    BufReader::new(asset_handle).read_to_end(&mut font_data).unwrap();
    let font_handle = ui.hui.add_font(&font_data);
    ui.hui.push_font(font_handle);
    storages.add_unique(UiFontPrefab(font_handle));
  }

  //log::info!("Compiling shaders...");
  // storages.add_unique_non_send_sync(ChunkShaderPrefab(
  //   include_shader_prefab!(
  //     "world",
  //     "../shaders/world.vert",
  //     "../shaders/world.frag",
  //     &renderer.display
  //   )
  // ));
  // storages.add_unique_non_send_sync(ColoredShaderPrefab(
  //   include_shader_prefab!(
  //     "colored",
  //     "../shaders/colored.vert",
  //     "../shaders/colored.frag",
  //     &renderer.display
  //   )
  // ));
  // storages.add_unique_non_send_sync(Colored2ShaderPrefab(
  //   include_shader_prefab!(
  //     "colored",
  //     "../shaders/colored2.vert",
  //     "../shaders/colored2.frag",
  //     &renderer.display
  //   )
  // ));

  //log::info!("releasing shader compiler");

  //renderer.display.release_shader_compiler();
}
