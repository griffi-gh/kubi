#version 300 es

precision highp float;
precision highp sampler2D;

out vec4 out_color;
in vec4 vtx_color;
in vec2 vtx_uv;
uniform sampler2D tex;

void main() {
  out_color = texture(tex, vtx_uv) * vtx_color;
}
