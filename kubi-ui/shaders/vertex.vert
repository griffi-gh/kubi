#version 300 es

precision highp float;

uniform vec2 resolution;
in vec4 color;
in vec2 position;
out vec4 vtx_color;

void main() {
  vtx_color = color;
  vec2 pos2d = (vec2(2., -2.) * (position / resolution)) + vec2(-1, 1);
  gl_Position = vec4(pos2d, 0., 1.);
}
