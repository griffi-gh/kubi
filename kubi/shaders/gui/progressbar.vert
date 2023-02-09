#version 300 es

in vec2 position;
out vec2 v_uv;
uniform mat4 ui_view;
uniform mat3 transform;

void main() {
  v_uv = position;
  vec2 transformed = (transform * vec3(position, 1.)).xy;
  gl_Position = ui_view * vec4(transformed, 0., 1.);
}
