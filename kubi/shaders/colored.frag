#version 300 es

precision highp float;

out vec4 out_color;
uniform vec4 color;

void main() {
  // discard fully transparent pixels
  if (color.w <= 0.) discard;
  out_color = color;
}
