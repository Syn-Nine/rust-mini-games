#version 100
precision mediump float;

varying vec4 v_color;
uniform vec4 color_uniform;

void main() {
    gl_FragColor = v_color * color_uniform;
}