#version 300 es

precision highp float;
precision highp sampler2D;

out vec4 out_color;
in vec4 vtx_color;
in vec2 vtx_uv;
uniform bool use_tex;
uniform sampler2D tex;

void main() {
  //if (vtx_color.w <= 0.) discard;
  vec4 tex_color;
  if (use_tex) {
    tex_color = texture(tex, vtx_uv);
  } else {
    tex_color = vec4(1.);
  }
  out_color = tex_color * vtx_color;
}
