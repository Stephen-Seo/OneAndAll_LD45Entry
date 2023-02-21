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
attribute vec2 origin;
attribute mat3 transform;

void main() {
  fragTexCoord = vertexTexCoord;
  fragColor = vertexColor;

  vec3 pos = transform * (vertexPosition - vec3(origin, 0.0)) + vec3(origin, 0.0);
  gl_Position = mvp * vec4(pos, 1.0);
}
