#version 150 core

in vec2 position;
in vec2 uv;
out vec2 v_uv;
uniform vec2 ui_scale;
uniform vec2 element_position;
uniform vec2 element_size;

void main() {
  v_uv = uv;
  gl_Position = vec4(ui_scale * (element_position + (position * element_size)), 0.0, 1.0);
}
