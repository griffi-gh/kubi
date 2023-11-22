#version 300 es

precision highp float;
in vec2 position;
uniform vec2 resolution;

void main() {
  gl_Position = vec4(vec2(1., -1.) * (position / resolution), 0., 1.);
}
