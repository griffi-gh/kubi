use shipyard::{Unique, UniqueView};
use crate::{
  prefabs::TexturePrefabs,
  rendering::{camera::CameraUniformBuffer, world::ChunkVertex, Renderer}
};

pub fn init_world_pipeline(
  ren: UniqueView<Renderer>,
  textures: UniqueView<TexturePrefabs>,
  camera_ubo: UniqueView<CameraUniformBuffer>,
) -> wgpu::RenderPipeline {
  let shader = ren.device().create_shader_module(
    wgpu::include_wgsl!("../../../shaders/world.wgsl")
  );

  let world_pipeline_layout = ren.device().create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
    label: Some("world_pipeline_layout"),
    bind_group_layouts: &[
      &textures.block_diffuse_bind_group_layout,
      &camera_ubo.camera_bind_group_layout,
    ],
    push_constant_ranges: &[],
  });

  ren.device().create_render_pipeline(&wgpu::RenderPipelineDescriptor {
    label: Some("world_pipeline"),
    layout: Some(&world_pipeline_layout),
    fragment: Some(wgpu::FragmentState {
      module: &shader,
      entry_point: "fs_main",
      compilation_options: wgpu::PipelineCompilationOptions::default(),
      targets: &[Some(wgpu::ColorTargetState {
        format: ren.surface_config().format,
        blend: Some(wgpu::BlendState::REPLACE),
        write_mask: wgpu::ColorWrites::ALL,
      })],
    }),
    vertex: wgpu::VertexState {
      module: &shader,
      entry_point: "vs_main",
      compilation_options: wgpu::PipelineCompilationOptions::default(),
      buffers: &[
        ChunkVertex::LAYOUT,
      ],
    },
    primitive: wgpu::PrimitiveState {
      topology: wgpu::PrimitiveTopology::TriangleList,
      strip_index_format: None,
      cull_mode: Some(wgpu::Face::Back),
      front_face: wgpu::FrontFace::Ccw,
      unclipped_depth: false,
      polygon_mode: wgpu::PolygonMode::Fill,
      conservative: false,
    },
    depth_stencil: None,
    multisample: wgpu::MultisampleState::default(),
    multiview: None,
  })
}
