#version 300 es

precision highp float;

uniform vec2 resolution;
in vec4 color;
in vec2 position;
out vec4 vtx_color;

void main() {
  vtx_color = color;
  gl_Position = vec4(vec2(1., -1.) * (position / resolution), 0., 1.);
}
