use glam::{Vec3, Mat4, Quat, ivec3};
use shipyard::{NonSendSync, UniqueView, UniqueViewMut, View, IntoIter, track, Unique, AllStoragesView};
use wgpu::util::DeviceExt;
use crate::{
  camera::Camera,
  player::MainPlayer,
  transform::Transform,
  assets::BlockTexturesAsset,
  world::{
    ChunkStorage, 
    ChunkMeshStorage, 
    chunk::CHUNK_SIZE,
  }, settings::GameSettings,
};
use super::{Renderer, RenderTarget, shaders::Shaders, camera_uniform::CameraUniformBuffer};

#[repr(C, packed)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ChunkVertex {
  pub position: [f32; 3],
  pub normal: [f32; 3],
  pub uv: [f32; 2],
  pub tex_index: u32,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Default, bytemuck::Pod, bytemuck::Zeroable)]
struct WorldUniform {
  position: [f32; 3], //XXX: should use i32?
}

///private type
#[derive(Unique)]
pub struct GpuData {
  pipeline: wgpu::RenderPipeline,
  uniform_buffer: wgpu::Buffer,
  bind_group: wgpu::BindGroup,
}

pub fn init_gpu_data(
  storages: AllStoragesView,
) {
  let renderer = storages.borrow::<UniqueView<Renderer>>().unwrap();
  let shaders: UniqueView<'_, Shaders> = storages.borrow::<UniqueView<Shaders>>().unwrap();
  let camera_uniform = storages.borrow::<UniqueView<CameraUniformBuffer>>().unwrap();

  let pipeline = renderer.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
    label: Some("WorldRenderPipeline"),
    layout: Some(&renderer.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
      label: Some("WorldRenderPipelineLayout"),
      bind_group_layouts: &[],
      push_constant_ranges: &[],
    })),
    vertex: wgpu::VertexState {
      module: &shaders.world,
      entry_point: "vs_main",
      buffers: &[],
    },
    fragment: Some(wgpu::FragmentState {
      module: &shaders.world,
      entry_point: "fs_main",
      targets: &[Some(wgpu::ColorTargetState {
        format: renderer.config.format,
        blend: Some(wgpu::BlendState::REPLACE),
        write_mask: wgpu::ColorWrites::ALL,
      })],
    }),
    primitive: wgpu::PrimitiveState {
      topology: wgpu::PrimitiveTopology::TriangleList,
      strip_index_format: None,
      front_face: wgpu::FrontFace::Ccw,
      cull_mode: Some(wgpu::Face::Back),
      polygon_mode: wgpu::PolygonMode::Fill,
      unclipped_depth: false,
      conservative: false,
    },
    //TODO enable depth buffer
    depth_stencil: None,
    multisample: wgpu::MultisampleState::default(),
    multiview: None,
  });

  let uniform_buffer = renderer.device.create_buffer_init(
    &wgpu::util::BufferInitDescriptor {
      label: Some("WorldUniformBuffer"),
      contents: bytemuck::cast_slice(&[WorldUniform::default()]),
      usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    }
  );

  let bind_group_layout = renderer.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
    entries: &[
      wgpu::BindGroupLayoutEntry {
        binding: 0,
        visibility: wgpu::ShaderStages::VERTEX,
        ty: wgpu::BindingType::Buffer {
          ty: wgpu::BufferBindingType::Uniform,
          has_dynamic_offset: false,
          min_binding_size: None,
        },
        count: None,
      },
      wgpu::BindGroupLayoutEntry {
        binding: 1,
        visibility: wgpu::ShaderStages::VERTEX,
        ty: wgpu::BindingType::Buffer {
          ty: wgpu::BufferBindingType::Uniform,
          has_dynamic_offset: false,
          min_binding_size: None,
        },
        count: None,
      }
    ],
    label: Some("WorldBindGroupLayout"),
  });
  let bind_group = renderer.device.create_bind_group(&wgpu::BindGroupDescriptor {
    layout: &bind_group_layout,
    entries: &[
      wgpu::BindGroupEntry {
        binding: 0,
        resource: camera_uniform.0.as_entire_binding(),
      },
      wgpu::BindGroupEntry {
        binding: 1,
        resource: uniform_buffer.as_entire_binding(),
      }
    ],
    label: Some("WorldBindGroup"),
  });

  storages.add_unique(GpuData { pipeline, uniform_buffer, bind_group });
}

pub fn draw_world(
  renderer: UniqueView<Renderer>,
  mut target: UniqueViewMut<RenderTarget>,
  gpu_data: UniqueView<GpuData>,
  chunks: UniqueView<ChunkStorage>,
  meshes: UniqueView<ChunkMeshStorage>,
  shaders: UniqueView<Shaders>,
  texture: UniqueView<BlockTexturesAsset>,
  settings: UniqueView<GameSettings>
) {
  for (&position, chunk) in &chunks.chunks {
    if let Some(key) = chunk.mesh_index {
      let mesh = meshes.get(key).expect("Mesh index pointing to nothing");
      let world_position = position.as_vec3() * CHUNK_SIZE as f32;
      
      //TODO culling like in the glium version

      //Draw chunk mesh
     
      //TODO: i need renderpass here!
    }
  }
}


// pub fn draw_world(
//   mut target: NonSendSync<UniqueViewMut<RenderTarget>>,
//   chunks: UniqueView<ChunkStorage>,
//   meshes: NonSendSync<UniqueView<ChunkMeshStorage>>,
//   program: NonSendSync<UniqueView<ChunkShaderPrefab>>,
//   texture: NonSendSync<UniqueView<BlockTexturesAsset>>,
//   camera: View<Camera>,
//   settings: UniqueView<GameSettings>
// ) {
//   let camera = camera.iter().next().expect("No cameras in the scene");
//   let draw_parameters = DrawParameters {
//     depth: Depth {
//       test: DepthTest::IfLess,
//       write: true,
//       ..Default::default()
//     },
//     multisampling: settings.msaa.is_some(),
//     polygon_mode: PolygonMode::Fill, //Change to Line for wireframe
//     backface_culling: BackfaceCullingMode::CullClockwise,
//     ..Default::default()
//   };
//   let texture_sampler = Sampler(&texture.0, SamplerBehavior {
//     minify_filter: MinifySamplerFilter::LinearMipmapLinear,
//     magnify_filter: MagnifySamplerFilter::Nearest,
//     max_anisotropy: settings.max_anisotropy.unwrap_or_default(),
//     wrap_function: (SamplerWrapFunction::Clamp, SamplerWrapFunction::Clamp, SamplerWrapFunction::Clamp),
//     ..Default::default()
//   });
//   let view = camera.view_matrix.to_cols_array_2d();
//   let perspective = camera.perspective_matrix.to_cols_array_2d();

//   for (&position, chunk) in &chunks.chunks {
//     if let Some(key) = chunk.mesh_index {
//       let mesh = meshes.get(key).expect("Mesh index pointing to nothing");
//       let world_position = position.as_vec3() * CHUNK_SIZE as f32;
      
//       //Skip mesh if its empty
//       if mesh.index_buffer.len() == 0 {
//         continue
//       }

//       //Frustum culling
//       {
//         let minp = world_position;
//         let maxp = world_position + Vec3::splat(CHUNK_SIZE as f32);
//         if !camera.frustum.is_box_visible(minp, maxp) {
//           continue
//         }
//       }

//       //Draw chunk mesh
//       target.0.draw(
//         &mesh.vertex_buffer,
//         &mesh.index_buffer,
//         &program.0,
//         &uniform! {
//           position_offset: world_position.to_array(),
//           view: view,
//           perspective: perspective,
//           tex: texture_sampler,
//         },
//         &draw_parameters
//       ).unwrap();
//     }
//   }
// }

pub fn draw_current_chunk_border() {}


// pub fn draw_current_chunk_border(
//   mut target: NonSendSync<UniqueViewMut<RenderTarget>>,
//   player: View<MainPlayer>,
//   transforms: View<Transform, track::All>,
//   buffers: UniqueView<CubePrimitive>,
//   program: NonSendSync<UniqueView<ColoredShaderPrefab>>,
//   camera: View<Camera>,
//   settings: UniqueView<GameSettings>,
// ) {
//   #[cfg(fuck)] {
//     if cfg!(target_os = "android") {
//       return
//     }
//     if !settings.debug_draw_current_chunk_border {
//       return
//     }
//     let camera = camera.iter().next().expect("No cameras in the scene");
//     let view = camera.view_matrix.to_cols_array_2d();
//     let perspective = camera.perspective_matrix.to_cols_array_2d();
//     let (_, &player_transform) = (&player, &transforms).iter().next().expect("No player");
//     let (_, _, player_position) = player_transform.0.to_scale_rotation_translation();
//     let player_in_chunk = ivec3(
//       (player_position.x as i32).div_euclid(CHUNK_SIZE as i32),
//       (player_position.y as i32).div_euclid(CHUNK_SIZE as i32),
//       (player_position.z as i32).div_euclid(CHUNK_SIZE as i32),
//     );
//     let world_position = player_in_chunk.as_vec3() * CHUNK_SIZE as f32;
//     target.0.draw(
//       &buffers.0,
//       &buffers.1,
//       &program.0,
//       &uniform! {
//         model: Mat4::from_scale_rotation_translation(
//           Vec3::splat(CHUNK_SIZE as f32),
//           Quat::default(),
//           world_position
//         ).to_cols_array_2d(),
//         color: [0.25f32; 4],
//         view: view,
//         perspective: perspective,
//       },
//       &DrawParameters {
//         depth: Depth {
//           test: DepthTest::IfLess,
//           ..Default::default()
//         },
//         blend: Blend::alpha_blending(),
//         ..Default::default()
//       }
//     ).unwrap();
//     target.0.draw(
//       &buffers.0,
//       &buffers.1,
//       &program.0,
//       &uniform! {
//         model: Mat4::from_scale_rotation_translation(
//           Vec3::splat(CHUNK_SIZE as f32),
//           Quat::default(),
//           world_position
//         ).to_cols_array_2d(),
//         color: [0.0f32; 4],
//         view: view,
//         perspective: perspective,
//       },
//       &DrawParameters {
//         polygon_mode: PolygonMode::Point,
//         line_width: Some(2.),
//         point_size: Some(5.),
//         ..Default::default()
//       }
//     ).unwrap();
//   }
// }
