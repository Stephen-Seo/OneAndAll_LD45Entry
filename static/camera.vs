#version 100

// Input vertex attributes
attribute vec3 vertexPosition;
attribute vec2 vertexTexCoord;
attribute vec4 vertexColor;

// Input uniform values
uniform mat4 mvp;

// Output vertex attributes (to fragment shader)
varying vec2 fragTexCoord;
varying vec4 fragColor;

// custom
attribute vec2 camera;

void main() {
  fragTexCoord = vertexTexCoord;
  fragColor = vertexColor;

  gl_Position = mvp * vec4(vertexPosition - vec3(camera, 0.0), 1.0);
}
