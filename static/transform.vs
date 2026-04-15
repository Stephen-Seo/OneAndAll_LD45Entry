#version 100

precision mediump float;

// Input vertex attributes
attribute vec3 vertexPosition;
attribute vec2 vertexTexCoord;
attribute vec4 vertexColor;

// Output vertex attributes (to fragment shader)
varying vec2 fragTexCoord;
varying vec4 fragColor;

// Input uniform values
uniform mat4 mvp;

// custom
uniform vec2 camera;
uniform vec2 origin;
uniform mat4 transform;

void main() {
  fragTexCoord = vertexTexCoord;
  fragColor = vertexColor;

  vec4 pos = transform * vec4((vertexPosition - vec3(origin, 0.0)), 0.0) + vec4(origin, 0.0, 0.0);
  gl_Position = mvp * vec4(vec2(pos) - camera, 0.0, 1.0);
}
