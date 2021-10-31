///////////////////////////////////////////////////////////////////////////////
/// Vertex Shader
///////////////////////////////////////////////////////////////////////////////
#[allow(dead_code)]
pub const VS_SRC: &'static [u8] = b"

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

\0";

///////////////////////////////////////////////////////////////////////////////
/// Fragment Shader
///////////////////////////////////////////////////////////////////////////////
#[allow(dead_code)]
pub const FS_SRC: &'static [u8] = b"

#version 100
precision mediump float;

varying vec4 v_color;
uniform vec4 color_uniform;

void main() {
    gl_FragColor = v_color * color_uniform;
}

\0";
