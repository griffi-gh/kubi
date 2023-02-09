#version 300 es

precision highp float;

in vec3 position;
uniform mat4 model;
uniform mat4 perspective;
uniform mat4 view;

void main() {
  mat4 modelview = view * model;
  gl_Position = perspective * modelview * vec4(position, 1.);
}
