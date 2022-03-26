#version 100
precision mediump float;

attribute vec2 position;
attribute vec4 color;

varying vec4 v_color;

uniform mat4 MVP;
uniform int alt;

void main() {

    if (0 == alt) {
        gl_Position = MVP * vec4(position.x, position.y, 0.0, 1.0);
    } else {
        gl_Position = MVP * vec4(position.x, 0.0, position.y, 1.0);
    }

    v_color = color;
}