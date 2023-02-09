#version 300 es

precision highp float;

in vec2 v_uv;
out vec4 out_color;
uniform float progress;
uniform vec4 color;
uniform vec4 bg_color;

void main() {
  if (v_uv.x <= progress) {
    out_color = color;
  } else {
    out_color = bg_color;
  }
}
