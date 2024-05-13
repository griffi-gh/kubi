struct CameraUniform {
  view_proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct SboxUniform {
  position: vec3<f32>,
};

@group(1) @binding(0)
var<uniform> sbox: SboxUniform;

@vertex
fn vs_main(
  @location(0) position: vec3<f32>,
) -> @builtin(position) vec4<f32> {
  return camera.view_proj * vec4<f32>(position + sbox.position, 1.0);
}

@fragment
fn fs_main(
  @builtin(position) in: vec4<f32>,
) -> @location(0) vec4<f32> {
  return vec4<f32>(0.0, 0.0, 0.0, 0.5);
}
