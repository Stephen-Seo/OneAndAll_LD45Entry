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
attribute vec2 camera;
attribute vec2 origin;
attribute mat3 transform;

void main() {
  fragTexCoord = vertexTexCoord;
  fragColor = vertexColor;

  vec3 pos = transform * (vertexPosition - vec3(origin, 0.0)) + vec3(origin, 0.0);
  gl_Position = mvp * vec4(pos - vec3(camera, 0.0), 1.0);
}
