#version 150 core

in vec2 position;
out vec2 uv;
uniform vec2 ui_view;
uniform vec2 element_position;
uniform vec2 element_size;

void main() {
  uv = position;
  gl_Position = vec4(ui_view * (element_position + (position * element_size)), 0.0, 1.0);
}
