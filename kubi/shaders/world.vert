#version 150 core

//TODO pack this data:
// uint position_normal_uv
// XXYYZZNU
// wehere Normal and Uv are enums
// maybe somehow pack in tex index too

in vec3 position;
in vec3 normal;
in vec2 uv;
in uint tex_index;
out vec2 v_uv;
out vec3 v_normal;
flat out uint v_tex_index;
uniform vec3 position_offset;
uniform mat4 perspective;
uniform mat4 view;

void main() {
  v_normal = normal;
  v_tex_index = tex_index;
  v_uv = uv;
  gl_Position = perspective * view * vec4(position + position_offset, 1.);
}
