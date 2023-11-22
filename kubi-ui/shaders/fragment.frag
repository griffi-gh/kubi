#version 300 es

precision highp float;

out vec4 out_color;
in vec4 vtx_color;

void main() {
  if (vtx_color.w <= 0.) discard;
  out_color = vtx_color;
}
