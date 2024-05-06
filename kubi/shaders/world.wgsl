struct CameraUniform {
  view_proj: mat4x4<f32>,
};

@group(1) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
  @location(0) position: vec3<f32>,
  @location(1) normal: vec3<f32>,
  @location(2) uv: vec2<f32>,
  @location(3) tex_index: u32,
}

struct VertexOutput {
  @builtin(position) clip_position: vec4<f32>,
  @location(0) uv: vec2<f32>,
  @location(1) normal: vec3<f32>,
  @location(2) @interpolate(flat)tex_index: u32,
};

@vertex
fn vs_main(
  in: VertexInput,
) -> VertexOutput {
  var out: VertexOutput;
  out.uv = in.uv;
  out.normal = in.normal;
  out.tex_index = in.tex_index;
  out.clip_position = camera.view_proj * vec4<f32>(in.position, 1.0);
  return out;
}

@group(0) @binding(0)
var t_diffuse: texture_2d_array<f32>;

@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
  let color: vec4<f32> = textureSample(t_diffuse, s_diffuse, in.uv, in.tex_index);
  if (color.a == 0.) {
    discard;
  }
  return color;
}
