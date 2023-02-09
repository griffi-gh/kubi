#version 150 core

in vec2 position;
out vec2 uv;
uniform mat4 ui_view;
uniform mat3 transform;

void main() {
  uv = position;
  vec2 transformed = (transform * vec3(position, 1.)).xy;
  gl_Position = ui_view * vec4(transformed, 0., 1.);
}
