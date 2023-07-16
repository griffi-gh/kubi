//TODO migrate, this is just some filler code to make the game compile

struct VertexOutput {
  @builtin(position) position: vec4<f32>,
};

@vertex
fn vs_main(
  @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
  var out: VertexOutput;
  out.position = vec4<f32>(0.0, 0.0, 0.0, 1.0);
  return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
  return vec4<f32>(0.3, 0.2, 0.1, 1.0);
}
