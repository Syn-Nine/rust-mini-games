mod line_shader;
mod poly_shader;
mod tex_shader;

use super::log;
use cgmath::*;
use glutin::{self, PossiblyCurrent};

pub mod gl {
    //pub use self::Gl as Gl;
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

pub struct Gl {
    pub gl: gl::Gl,
    font_shader: Shader,
    line_shader: Shader,
    poly_shader: Shader,
    tex_shader: Shader,
    texture: Texture,
    xres: f32,
    yres: f32,
}

impl Gl {
    pub fn gen_vao(&self) -> u32 {
        let mut vao: u32 = 0;
        unsafe {
            self.gl.GenVertexArrays(1, &mut vao);
        }
        vao
    }

    pub fn gen_vbo(&self) -> u32 {
        let mut vbo: u32 = 0;
        unsafe {
            self.gl.GenBuffers(1, &mut vbo);
        }
        vbo
    }

    /*pub fn bind_vao(&self, vao: u32) {
        unsafe {
            self.gl.BindVertexArray(vao);
        }
    }

    pub fn bind_vbo(&self, vbo: u32) {
        unsafe {
            self.gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
        }
    }*/

    pub fn load_texture(&self, image: &String) -> u32 {
        Texture::new(&self.gl, image).handle
    }

    pub fn buffer_font_data(
        &self,
        vao: u32,
        vbo: u32,
        num_chars: usize,
        data_ptr: *const std::ffi::c_void,
    ) {
        unsafe {
            self.gl.BindVertexArray(vao);
            self.gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
            self.gl.BufferData(
                gl::ARRAY_BUFFER,
                (num_chars * 2 * 3 * 4 * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                data_ptr,
                gl::STATIC_DRAW,
            );

            self.gl.EnableVertexAttribArray(self.font_shader.attrib_pos);
            self.gl.VertexAttribPointer(
                self.font_shader.attrib_pos,
                2,
                gl::FLOAT,
                0,
                4 * std::mem::size_of::<f32>() as gl::types::GLsizei,
                std::ptr::null(),
            );

            self.gl.EnableVertexAttribArray(self.font_shader.attrib_uv);
            self.gl.VertexAttribPointer(
                self.font_shader.attrib_uv,
                2,
                gl::FLOAT,
                0,
                4 * std::mem::size_of::<f32>() as gl::types::GLsizei,
                (2 * std::mem::size_of::<f32>()) as *const () as *const _,
            );
        }
    }

    pub fn buffer_billboard_data(&self, vao: u32, vbo: u32, data_ptr: *const std::ffi::c_void) {
        unsafe {
            self.gl.BindVertexArray(vao);
            self.gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
            self.gl.BufferData(
                gl::ARRAY_BUFFER,
                (2 * 3 * 4 * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                data_ptr,
                gl::STATIC_DRAW,
            );

            self.gl.EnableVertexAttribArray(self.tex_shader.attrib_pos);
            self.gl.VertexAttribPointer(
                self.tex_shader.attrib_pos,
                2,
                gl::FLOAT,
                0,
                4 * std::mem::size_of::<f32>() as gl::types::GLsizei,
                std::ptr::null(),
            );

            self.gl.EnableVertexAttribArray(self.tex_shader.attrib_uv);
            self.gl.VertexAttribPointer(
                self.tex_shader.attrib_uv,
                2,
                gl::FLOAT,
                0,
                4 * std::mem::size_of::<f32>() as gl::types::GLsizei,
                (2 * std::mem::size_of::<f32>()) as *const () as *const _,
            );
        }
    }

    pub fn buffer_line_data(
        &self,
        vao: u32,
        vbo: u32,
        num_lines: usize,
        data_ptr: *const std::ffi::c_void,
    ) {
        unsafe {
            self.gl.BindVertexArray(vao);
            self.gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
            self.gl.BufferData(
                gl::ARRAY_BUFFER,
                (num_lines * 2 * 6 * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                data_ptr,
                gl::STATIC_DRAW,
            );

            self.gl.EnableVertexAttribArray(self.line_shader.attrib_pos);
            self.gl.VertexAttribPointer(
                self.line_shader.attrib_pos,
                2,
                gl::FLOAT,
                0,
                6 * std::mem::size_of::<f32>() as gl::types::GLsizei,
                std::ptr::null(),
            );

            self.gl
                .EnableVertexAttribArray(self.line_shader.attrib_color);
            self.gl.VertexAttribPointer(
                self.line_shader.attrib_color,
                4,
                gl::FLOAT,
                0,
                6 * std::mem::size_of::<f32>() as gl::types::GLsizei,
                (2 * std::mem::size_of::<f32>()) as *const () as *const _,
            );
        }
    }

    pub fn buffer_triangle_data(
        &self,
        vao: u32,
        vbo: u32,
        num_triangles: usize,
        data_ptr: *const std::ffi::c_void,
    ) {
        unsafe {
            self.gl.BindVertexArray(vao);
            self.gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
            self.gl.BufferData(
                gl::ARRAY_BUFFER,
                (num_triangles * 3 * 6 * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                data_ptr,
                gl::STATIC_DRAW,
            );

            self.gl.EnableVertexAttribArray(self.poly_shader.attrib_pos);
            self.gl.VertexAttribPointer(
                self.poly_shader.attrib_pos,
                2,
                gl::FLOAT,
                0,
                6 * std::mem::size_of::<f32>() as gl::types::GLsizei,
                std::ptr::null(),
            );

            self.gl
                .EnableVertexAttribArray(self.poly_shader.attrib_color);
            self.gl.VertexAttribPointer(
                self.poly_shader.attrib_color,
                4,
                gl::FLOAT,
                0,
                6 * std::mem::size_of::<f32>() as gl::types::GLsizei,
                (2 * std::mem::size_of::<f32>()) as *const () as *const _,
            );
        }
    }
}

struct Texture {
    handle: u32,
}

impl Texture {
    pub fn new(gl: &gl::Gl, image: &String) -> Texture {
        unsafe {
            // Construct a new RGB ImageBuffer with the specified width and height.
            log(format!("Texture: Loading '{}'", image));
            let img: image::RgbaImage = image::open(image).unwrap().to_rgba8();

            let mut tex: u32 = 0;
            gl.GenTextures(1, &mut tex);
            gl.BindTexture(gl::TEXTURE_2D, tex);
            let tw = img.dimensions().0 as gl::types::GLsizei;
            let th = img.dimensions().1 as gl::types::GLsizei;
            gl.TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as gl::types::GLint,
                tw,
                th,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                img.into_raw().as_ptr() as *const _,
            );

            gl.TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::NEAREST as gl::types::GLint,
            );
            gl.TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MAG_FILTER,
                gl::NEAREST as gl::types::GLint,
            );
            gl.TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_S,
                gl::REPEAT as gl::types::GLint,
            );
            gl.TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_T,
                gl::REPEAT as gl::types::GLint,
            );

            Texture { handle: tex }
        }
    }
}

struct Shader {
    program: u32,
    pub attrib_pos: gl::types::GLuint,
    pub attrib_color: gl::types::GLuint,
    pub attrib_uv: gl::types::GLuint,
    pub uniform_tex_sampler: gl::types::GLint,
    pub uniform_mvp: gl::types::GLint,
    pub uniform_color: gl::types::GLint,
}

impl Shader {
    pub fn new(gl: &gl::Gl, vs_src: &'static [u8], fs_src: &'static [u8]) -> Shader {
        unsafe {
            let vs = gl.CreateShader(gl::VERTEX_SHADER);
            gl.ShaderSource(
                vs,
                1,
                [vs_src.as_ptr() as *const _].as_ptr(),
                std::ptr::null(),
            );
            gl.CompileShader(vs);

            let fs = gl.CreateShader(gl::FRAGMENT_SHADER);
            gl.ShaderSource(
                fs,
                1,
                [fs_src.as_ptr() as *const _].as_ptr(),
                std::ptr::null(),
            );
            gl.CompileShader(fs);

            let program = gl.CreateProgram();
            gl.AttachShader(program, vs);
            gl.AttachShader(program, fs);
            gl.LinkProgram(program);

            let attrib_pos = gl.GetAttribLocation(program, b"position\0".as_ptr() as *const _)
                as gl::types::GLuint;
            let attrib_color =
                gl.GetAttribLocation(program, b"color\0".as_ptr() as *const _) as gl::types::GLuint;
            let attrib_uv =
                gl.GetAttribLocation(program, b"uv\0".as_ptr() as *const _) as gl::types::GLuint;

            let uniform_tex_sampler =
                gl.GetUniformLocation(program, b"tex_sampler\0".as_ptr() as *const _);
            let uniform_mvp = gl.GetUniformLocation(program, b"MVP\0".as_ptr() as *const _);

            let uniform_color =
                gl.GetUniformLocation(program, b"color_uniform\0".as_ptr() as *const _);

            Shader {
                program,
                attrib_pos,
                attrib_color,
                attrib_uv,
                uniform_tex_sampler,
                uniform_mvp,
                uniform_color,
            }
        }
    }

    pub fn use_program(&self, gl: &gl::Gl) {
        unsafe {
            gl.UseProgram(self.program);
        }
    }
}

pub fn load(gl_context: &glutin::Context<PossiblyCurrent>, xres: i32, yres: i32) -> Gl {
    let gl = gl::Gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);

    let line_shader = Shader::new(&gl, line_shader::VS_SRC, line_shader::FS_SRC);
    let poly_shader = Shader::new(&gl, poly_shader::VS_SRC, poly_shader::FS_SRC);
    let font_shader = Shader::new(&gl, tex_shader::VS_SRC, tex_shader::FS_SRC);
    let tex_shader = Shader::new(&gl, tex_shader::VS_SRC, tex_shader::FS_SRC);

    let texture = Texture::new(&gl, &String::from("assets/retro_gaming_0.png"));

    unsafe {
        let color = [0.05, 0.05, 0.06, 1.0];
        gl.ClearColor(color[0], color[1], color[2], color[3]);
        gl.Enable(gl::BLEND);
        gl.BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl.Enable(gl::MULTISAMPLE);
        gl.Enable(gl::LINE_SMOOTH);

        Gl {
            gl,
            font_shader,
            line_shader,
            poly_shader,
            tex_shader,
            texture,
            xres: xres as f32,
            yres: yres as f32,
        }
    }
}

impl Gl {
    pub fn clear_frame(&self) {
        unsafe {
            self.gl.Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn draw_text(
        &self,
        x: f32,
        y: f32,
        angle: f32,
        sx: f32,
        sy: f32,
        vao: u32,
        count: usize,
        color: super::ecs::Color,
    ) {
        self.font_shader.use_program(&self.gl);

        unsafe {
            self.gl.ActiveTexture(gl::TEXTURE0);
            self.gl.BindTexture(gl::TEXTURE_2D, self.texture.handle);
            self.gl.Uniform1i(self.font_shader.uniform_tex_sampler, 0);
            self.gl.Uniform4f(
                self.font_shader.uniform_color,
                color.r,
                color.g,
                color.b,
                color.a,
            );

            self.gl.BindVertexArray(vao);
            let mvp = self.get_mvp();

            let mat = Matrix4::from_translation(Vector3::new(x, y, 0.0));
            let mvp = mvp * mat;

            let mat = Matrix4::from_angle_z(cgmath::Rad(angle));
            let mvp = mvp * mat;

            let mat = Matrix4::from_nonuniform_scale(sx, sy, 1.0);
            let mvp = mvp * mat;

            self.gl.UniformMatrix4fv(
                self.font_shader.uniform_mvp,
                1,
                gl::FALSE,
                mvp.as_ptr() as *const _,
            );

            self.gl.DrawArrays(gl::TRIANGLES, 0, (count * 2 * 3) as i32);

            self.gl.BindVertexArray(0);
        }
    }

    pub fn draw_billboard(
        &self,
        x: f32,
        y: f32,
        angle: f32,
        sx: f32,
        sy: f32,
        vao: u32,
        tex: u16,
        color: super::ecs::Color,
    ) {
        self.tex_shader.use_program(&self.gl);

        unsafe {
            self.gl.ActiveTexture(gl::TEXTURE0);
            self.gl.BindTexture(gl::TEXTURE_2D, tex as u32);
            self.gl.Uniform1i(self.tex_shader.uniform_tex_sampler, 0);
            self.gl.Uniform4f(
                self.tex_shader.uniform_color,
                color.r,
                color.g,
                color.b,
                color.a,
            );

            self.gl.BindVertexArray(vao);
            let mvp = self.get_mvp();

            let mat = Matrix4::from_translation(Vector3::new(x, y, 0.0));
            let mvp = mvp * mat;

            let mat = Matrix4::from_angle_z(cgmath::Rad(angle));
            let mvp = mvp * mat;

            let mat = Matrix4::from_nonuniform_scale(sx, sy, 1.0);
            let mvp = mvp * mat;

            self.gl.UniformMatrix4fv(
                self.tex_shader.uniform_mvp,
                1,
                gl::FALSE,
                mvp.as_ptr() as *const _,
            );

            self.gl.DrawArrays(gl::TRIANGLES, 0, 6 as i32);

            self.gl.BindVertexArray(0);
        }
    }

    pub fn draw_lines(
        &self,
        x: f32,
        y: f32,
        angle: f32,
        sx: f32,
        sy: f32,
        vao: u32,
        count: usize,
        color: super::ecs::Color,
    ) {
        self.line_shader.use_program(&self.gl);

        unsafe {
            self.gl.Uniform4f(
                self.line_shader.uniform_color,
                color.r,
                color.g,
                color.b,
                color.a,
            );

            self.gl.BindVertexArray(vao);
            let mvp = self.get_mvp();

            let mat = Matrix4::from_translation(Vector3::new(x, y, 0.0));
            let mvp = mvp * mat;

            let mat = Matrix4::from_angle_z(cgmath::Rad(angle));
            let mvp = mvp * mat;

            let mat = Matrix4::from_nonuniform_scale(sx, sy, 1.0);
            let mvp = mvp * mat;

            self.gl.UniformMatrix4fv(
                self.line_shader.uniform_mvp,
                1,
                gl::FALSE,
                mvp.as_ptr() as *const _,
            );

            self.gl.DrawArrays(gl::LINES, 0, (count * 2) as i32);

            self.gl.BindVertexArray(0);
        }
    }

    pub fn draw_triangles(
        &self,
        x: f32,
        y: f32,
        angle: f32,
        sx: f32,
        sy: f32,
        vao: u32,
        count: usize,
        color: super::ecs::Color,
    ) {
        self.poly_shader.use_program(&self.gl);

        unsafe {
            self.gl.Uniform4f(
                self.poly_shader.uniform_color,
                color.r,
                color.g,
                color.b,
                color.a,
            );

            self.gl.BindVertexArray(vao);
            let mvp = self.get_mvp();

            let mat = Matrix4::from_translation(Vector3::new(x, y, 0.0));
            let mvp = mvp * mat;

            let mat = Matrix4::from_angle_z(cgmath::Rad(angle));
            let mvp = mvp * mat;

            let mat = Matrix4::from_nonuniform_scale(sx, sy, 1.0);
            let mvp = mvp * mat;

            self.gl.UniformMatrix4fv(
                self.poly_shader.uniform_mvp,
                1,
                gl::FALSE,
                mvp.as_ptr() as *const _,
            );

            self.gl.DrawArrays(gl::TRIANGLES, 0, (count * 3) as i32);

            self.gl.BindVertexArray(0);
        }
    }

    fn get_mvp(&self) -> Matrix4<f32> {
        let xr = 2.0 / self.xres;
        let yr = 2.0 / self.yres;

        #[rustfmt::skip]
        let mvp: Matrix4<f32> = Matrix4::new(
            xr, 0.0, 0.0, 0.0,
            0.0, -yr, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            -1.0, 1.0, 0.0, 1.0
        );

        mvp
    }
}
