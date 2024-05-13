pub fn create_render_pipeline(
  ren: UniqueView<Renderer>,
  stri_primitive: UniqueView<FstriPrimitive>,
) -> wgpu::RenderPipeline {
  let shader = ren.device().create_shader_module(
    wgpu::include_wgsl!("../../../shaders/c2d.wgsl")
  );
  let pipeline_layout = ren.device().create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
    label: Some("smoverlay_pipeline_layout"),
    bind_group_layouts: &[&ren.bind_group_layout],
    push_constant_ranges: &[],
  });
  //TODO
}
