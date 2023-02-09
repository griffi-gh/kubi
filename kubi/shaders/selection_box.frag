#version 300 es

precision highp float;

out vec4 color;
uniform vec4 u_color;

void main() {
  color = u_color;
  color -= vec4(0, 0, 0, 0.1 * sin(gl_FragCoord.x) * cos(gl_FragCoord.y));
}
