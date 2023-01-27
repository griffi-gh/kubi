#version 150 core

in vec3 v_normal;
in vec2 v_uv;
flat in uint v_tex_index;
out vec4 color;
uniform sampler2DArray tex;
uniform bool debug;

void main() {
  // base color from texture
  color = texture(tex, vec3(v_uv, v_tex_index));
  //basic "lighting"
  float light = abs(v_normal.x) + .8 * abs(v_normal.y) + .6 * abs(v_normal.z);
  color *= vec4(vec3(light), 1.);
  //highlight
  if (debug) {
    color *= vec4(1., 0., 0., 1.);
  }
}
