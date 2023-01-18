#version 150 core

in vec3 position;
in vec3 normal;
in vec2 uv;
in uint tex_index;
out vec2 v_uv;
out vec3 v_normal;
flat out uint v_tex_index;
uniform vec2 position_offset;
uniform mat4 perspective;
uniform mat4 view;

void main() {
  v_normal = normal;
  v_tex_index = tex_index;
  v_uv = uv;
  gl_Position = perspective * view * (vec4(position, 1.0) + vec4(position_offset.x, 0., position_offset.y, 0.));
}
