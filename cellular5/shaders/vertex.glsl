#version 450 core

in vec4 aPos;
in vec2 aTexCoord;

void main() {
    gl_Position = aPos;
    gl_ClipDistance[0] = 1.0;
}
