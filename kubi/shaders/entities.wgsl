struct CameraUniform {
  view_proj: mat4x4<f32>,
};

@group(1) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
  @location(0) uv: vec2<f32>,
  @location(1) position: vec3<f32>,
  @location(2) normal: vec3<f32>,
  @location(3) mat_row0: vec4<f32>,
  @location(4) mat_row1: vec4<f32>,
  @location(5) mat_row2: vec4<f32>,
  @location(6) mat_row3: vec4<f32>,
}

struct VertexOutput {
  @builtin(position) clip_position: vec4<f32>,
  @location(0) uv: vec2<f32>,
  @location(1) normal: vec3<f32>,
};

@vertex
fn vs_main(
  in: VertexInput,
) -> VertexOutput {
  let inst_mat = mat4x4<f32>(
    in.mat_row0,
    in.mat_row1,
    in.mat_row2,
    in.mat_row3,
  );
  var out: VertexOutput;
  out.clip_position = camera.view_proj * (inst_mat * vec4<f32>(in.position, 1.0));
  out.uv = in.uv;
  out.normal = in.normal;
  return out;
}

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;

@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
  return textureSample(t_diffuse, s_diffuse, in.uv);
}
