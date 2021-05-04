///////////////////////////////////////////////////////////////////////////////
/// Vertex Shader
///////////////////////////////////////////////////////////////////////////////
#[allow(dead_code)]
pub const VS_SRC: &'static [u8] = b"

#version 100
precision mediump float;

attribute vec2 position;
attribute vec2 uv;

varying vec2 v_uv;

uniform mat4 MVP;

void main() {
    gl_Position = MVP * vec4(position, 0.0, 1.0);
    v_uv = uv;
}

\0";

///////////////////////////////////////////////////////////////////////////////
/// Fragment Shader
///////////////////////////////////////////////////////////////////////////////
#[allow(dead_code)]
pub const FS_SRC: &'static [u8] = b"

#version 100
precision mediump float;

varying vec2 v_uv;

uniform sampler2D tex_sampler;
uniform vec4 color_uniform;

void main() {
    vec4 color = texture2D(tex_sampler, v_uv).rgba * color_uniform;

    gl_FragColor = color;
}

\0";
