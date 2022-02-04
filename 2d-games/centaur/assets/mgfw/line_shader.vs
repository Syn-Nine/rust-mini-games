#version 100
precision mediump float;

attribute vec2 position;
attribute vec4 color;

varying vec4 v_color;

uniform mat4 MVP;

void main() {
    gl_Position = MVP * vec4(position, 0.0, 1.0);
    v_color = color;
}