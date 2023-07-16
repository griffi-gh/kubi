use shipyard::{Unique, AllStoragesView, NonSendSync, UniqueView};
use super::{shaders::Shaders, Renderer};

#[derive(Unique)]
pub struct Pipelines {
  pub world: wgpu::RenderPipeline
}

pub fn init_pipelines(
  storages: AllStoragesView
) {
  let renderer = storages.borrow::<NonSendSync<UniqueView<Renderer>>>().unwrap();
  let shaders = storages.borrow::<UniqueView<Shaders>>().unwrap();
  storages.add_unique(Pipelines {
    world: renderer.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
    }),
  })
}
