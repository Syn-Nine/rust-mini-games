#version 100
precision mediump float;

varying vec2 v_uv;

uniform sampler2D tex_sampler;
uniform vec4 color_uniform;

void main() {
    vec4 color = texture2D(tex_sampler, v_uv).rgba * color_uniform;

    gl_FragColor = vec4(1.0, 1.0, 0.0, 1.0); //color;
    gl_FragColor = color;
}
