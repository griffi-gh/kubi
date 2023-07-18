struct Uniform {
  position_offset: vec3<f32>,
  view_proj: mat4x4<f32>,
};

@group(1) @binding(0)
var<uniform> world: Uniform;

struct VertexInput {
  @location(0) position: vec3<f32>,
  @location(1) normal: vec3<f32>,
  @location(2) uv: vec2<f32>,
  @location(3) tex_index: u32,
}

struct VertexOutput {
  @builtin(position) clip_position: vec4<f32>,
  @location(0) tex_coords: vec2<f32>,
}

@vertex
fn vs_main(
  model: VertexInput,
) -> VertexOutput {
  var out: VertexOutput;
  out.tex_coords = model.tex_coords;
  out.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0);
  return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
  return vec4(1.0, 0.0, 0.0, 0.0);
}
