struct CameraUniform {
  view_proj: mat4x4<f32>,
};

struct WorldUniform {
  position: vec3<f32>,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

@group(0) @binding(1)
var<uniform> world: WorldUniform;

struct VertexInput {
  @location(0) position: vec3<f32>,
  @location(1) normal: vec3<f32>,
  @location(2) tex_coords: vec2<f32>,
  @location(3) tex_index: u32,
}

struct VertexOutput {
  @builtin(position) clip_position: vec4<f32>,
  //@location(0) tex_coords: vec2<f32>,
}

@vertex
fn vs_main(
  model: VertexInput,
) -> VertexOutput {
  var out: VertexOutput;
  out.clip_position = camera.view_proj * vec4<f32>(model.position + world.position, 1.0);
  //out.tex_coords = model.tex_coords;
  return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
  return vec4(1.0, 0.0, 0.0, 0.0);
}
