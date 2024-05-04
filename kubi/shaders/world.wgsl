// struct Uniforms {
//   transform: mat4x4<f32>;
// };

// @group(1) @binding(0)
// var<uniform> uniforms: Uniforms;

struct VertexInput {
  @location(0) position: vec3<f32>,
  @location(1) normal: vec3<f32>,
  @location(2) uv: vec2<f32>,
  @location(3) @interpolate(flat) tex_index: u32,
}

struct VertexOutput {
  @builtin(position) clip_position: vec4<f32>,
  @location(0) uv: vec2<f32>,
  @location(1) normal: vec3<f32>,
  @location(2) color: vec4<f32>,
  @location(3) @interpolate(flat) tex_index: u32,
};

@vertex
fn vs_main(
  in: VertexInput,
) -> VertexOutput {
  var out: VertexOutput;
  out.uv = in.uv;
  out.clip_position = vec4<f32>(in.position, 1.0);
  return out;
}

@group(0) @binding(0)
var t_diffuse: texture_2d_array<f32>;

@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
  return textureSample(t_diffuse, s_diffuse, in.uv, in.tex_index);
}
