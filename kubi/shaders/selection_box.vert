#version 150 core

in vec3 position;
uniform ivec3 u_position;
uniform mat4 perspective;
uniform mat4 view;

void main() {
  gl_Position = perspective * view * vec4(position + vec3(u_position), 1.);
}
