#version 300 es

precision highp float;
precision lowp sampler2DArray;

in vec3 v_normal;
in vec2 v_uv;
flat in uint v_tex_index;
out vec4 color;
uniform sampler2DArray tex;

void main() {
  // base color from texture
  color = texture(tex, vec3(v_uv, v_tex_index));
  // discard fully transparent pixels
  if (color.w <= 0.0) {
    discard;
  }
  //basic "lighting"
  float light = abs(v_normal.x) + .8 * abs(v_normal.y) + .6 * abs(v_normal.z);
  color *= vec4(vec3(light), 1.);
}
