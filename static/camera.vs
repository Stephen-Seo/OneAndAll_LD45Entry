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

void main() {
  fragTexCoord = vertexTexCoord;
  fragColor = vertexColor;

  gl_Position = mvp * vec4(vertexPosition - vec3(camera, 0.0), 1.0);
}
