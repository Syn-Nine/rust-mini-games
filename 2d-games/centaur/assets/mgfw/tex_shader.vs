#version 100
precision mediump float;

attribute vec2 position;
attribute vec2 uv;

varying vec2 v_uv;

uniform mat4 MVP;
uniform int uniform_override_uv;
uniform vec2 uniform_uv;
uniform vec2 uniform_duv;

void main() {
    gl_Position = MVP * vec4(position, 0.0, 1.0);
    
    vec2 nuv = uv;
    if (1 == uniform_override_uv) {
        if (uv.x < 1.0e-6) {
            nuv.x = uniform_uv.x;
        }
        else if (uv.x > 1.0 - 1.0e-6) {
            nuv.x = uniform_uv.x + uniform_duv.x;
        }
        if (uv.y < 1.0e-6) {
            nuv.y = uniform_uv.y;
        }
        else if (uv.y > 1.0 - 1.0e-6) {
            nuv.y = uniform_uv.y + uniform_duv.y;
        }
    }

    v_uv = nuv;
}
