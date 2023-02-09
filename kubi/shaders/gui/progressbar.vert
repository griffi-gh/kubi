#version 150 core

in vec2 position;
out vec2 uv;
uniform mat4 ui_view;
uniform mat3 transform;

//not sure if mat4(i) does the same thing
mat4 extend(mat3 i) {
  mat4 o;
  o[0] = vec4(i[0].xyz, 0);
  o[1] = vec4(i[1].xyz, 0);
  o[2] = vec4(i[2].xyz, 0);
  o[3] = vec4(0, 0, 0, 1);
  return o;
}

void main() {
  uv = position;
  gl_Position = ui_view * extend(transform) * vec4(position, 0., 1.);
}
