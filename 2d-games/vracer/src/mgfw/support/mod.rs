use super::log;
use cgmath::*;
use glutin::{self, PossiblyCurrent};
use std::ffi::CString;
use std::fs::File;
use std::io::prelude::*;

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
    fbo: u32,
    colorbuf: u32,
    window_scale: f32,
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
            self.gl.BindVertexArray(0);
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

    pub fn buffer_tilemap_data(
        &self,
        vao: u32,
        vbo: u32,
        num_tiles: usize,
        data_ptr: *const std::ffi::c_void,
    ) {
        unsafe {
            self.gl.BindVertexArray(vao);
            self.gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
            self.gl.BufferData(
                gl::ARRAY_BUFFER,
                (num_tiles * 2 * 3 * 4 * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
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
    pub uniform_uv: gl::types::GLint,
    pub uniform_duv: gl::types::GLint,
    pub uniform_override_uv: gl::types::GLint,
    pub uniform_alt: gl::types::GLint,
}

impl Shader {
    pub fn new(gl: &gl::Gl, vs_src: &String, fs_src: &String) -> Shader {
        unsafe {
            let vs = gl.CreateShader(gl::VERTEX_SHADER);

            println!("Loading Vertex Shader: {}", vs_src);
            let mut vsfile = File::open(vs_src.as_str()).unwrap();
            let mut vsbuffer: Vec<u8> = Vec::new();
            vsfile.read_to_end(&mut vsbuffer).unwrap();
            let raw = CString::new(vsbuffer).unwrap();
            gl.ShaderSource(vs, 1, &raw.as_ptr(), std::ptr::null());

            println!("Compile Vertex Shader: {}", vs_src);
            gl.CompileShader(vs);

            let mut pass = i32::from(gl::FALSE);
            let mut loglen: i32 = 0;
            gl.GetShaderiv(vs, gl::INFO_LOG_LENGTH, &mut loglen);
            let mut output = Vec::<u8>::with_capacity(loglen as usize + 1);
            output.set_len(loglen as usize);

            gl.GetShaderiv(vs, gl::COMPILE_STATUS, &mut pass);
            if i32::from(gl::TRUE) != pass {
                gl.GetShaderInfoLog(
                    vs,
                    5120,
                    std::ptr::null_mut(),
                    output.as_mut_ptr() as *mut gl::types::GLchar,
                );
                println!(
                    "Compiler Error:\n {}",
                    std::str::from_utf8(&output).unwrap()
                );
            }

            let fs = gl.CreateShader(gl::FRAGMENT_SHADER);

            println!("Loading Fragment Shader: {}", fs_src);
            let mut fsfile = File::open(fs_src.as_str()).unwrap();
            let mut fsbuffer: Vec<u8> = Vec::new();
            fsfile.read_to_end(&mut fsbuffer).unwrap();
            let raw = CString::new(fsbuffer).unwrap();
            gl.ShaderSource(fs, 1, &raw.as_ptr(), std::ptr::null());

            println!("Compile Fragment Shader: {}", fs_src);
            gl.CompileShader(fs);

            let mut pass = i32::from(gl::FALSE);
            let mut loglen: i32 = 0;
            gl.GetShaderiv(vs, gl::INFO_LOG_LENGTH, &mut loglen);
            let mut output = Vec::<u8>::with_capacity(loglen as usize + 1);
            output.set_len(loglen as usize);

            gl.GetShaderiv(fs, gl::COMPILE_STATUS, &mut pass);
            if i32::from(gl::TRUE) != pass {
                gl.GetShaderInfoLog(
                    vs,
                    512,
                    std::ptr::null_mut(),
                    output.as_mut_ptr() as *mut gl::types::GLchar,
                );
                println!(
                    "Compiler Error:\n {}",
                    std::str::from_utf8(&output).unwrap()
                );
            }

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

            let uniform_uv = gl.GetUniformLocation(program, b"uniform_uv\0".as_ptr() as *const _);
            let uniform_duv = gl.GetUniformLocation(program, b"uniform_duv\0".as_ptr() as *const _);
            let uniform_override_uv =
                gl.GetUniformLocation(program, b"uniform_override_uv\0".as_ptr() as *const _);

            let uniform_alt = gl.GetUniformLocation(program, b"alt\0".as_ptr() as *const _);

            Shader {
                program,
                attrib_pos,
                attrib_color,
                attrib_uv,
                uniform_tex_sampler,
                uniform_mvp,
                uniform_color,
                uniform_uv,
                uniform_duv,
                uniform_override_uv,
                uniform_alt,
            }
        }
    }

    pub fn use_program(&self, gl: &gl::Gl) {
        unsafe {
            gl.UseProgram(self.program);
        }
    }
}

pub fn load(
    gl_context: &glutin::Context<PossiblyCurrent>,
    xres: i32,
    yres: i32,
    window_scale: f32,
) -> Gl {
    let gl = gl::Gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);

    let line_shader = Shader::new(
        &gl,
        &String::from("assets/mgfw/line_shader.vs"),
        &String::from("assets/mgfw/line_shader.fs"),
    );
    let poly_shader = Shader::new(
        &gl,
        &String::from("assets/mgfw/poly_shader.vs"),
        &String::from("assets/mgfw/poly_shader.fs"),
    );
    let font_shader = Shader::new(
        &gl,
        &String::from("assets/mgfw/tex_shader.vs"),
        &String::from("assets/mgfw/tex_shader.fs"),
    );
    let tex_shader = Shader::new(
        &gl,
        &String::from("assets/mgfw/tex_shader.vs"),
        &String::from("assets/mgfw/tex_shader.fs"),
    );

    // todo - pull this from the retro_gaming.rs
    let texture = Texture::new(&gl, &String::from("assets/mgfw/retro_gaming_0.png"));

    unsafe {
        gl.Viewport(0, 0, xres, yres);

        //let color = [0.05, 0.05, 0.06, 1.0];
        let color = [0.0, 0.0, 0.0, 1.0];
        gl.ClearColor(color[0], color[1], color[2], color[3]);
        gl.Enable(gl::BLEND);
        gl.BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl.Enable(gl::MULTISAMPLE);
        gl.Enable(gl::LINE_SMOOTH);

        // setup framebuffer object
        let mut fbo: u32 = 0;
        gl.GenFramebuffers(1, &mut fbo);
        gl.BindFramebuffer(gl::FRAMEBUFFER, fbo);

        let mut colorbuf: u32 = 0;
        gl.GenTextures(1, &mut colorbuf);
        gl.BindTexture(gl::TEXTURE_2D, colorbuf);
        gl.TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as gl::types::GLint,
            xres,
            yres,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            std::ptr::null(),
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
        gl.FramebufferTexture2D(
            gl::FRAMEBUFFER,
            gl::COLOR_ATTACHMENT0,
            gl::TEXTURE_2D,
            colorbuf,
            0,
        );

        let mut rbo: u32 = 0;
        gl.GenRenderbuffers(1, &mut rbo);
        gl.BindRenderbuffer(gl::RENDERBUFFER, rbo);
        gl.RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH24_STENCIL8, xres, yres);
        gl.FramebufferRenderbuffer(
            gl::FRAMEBUFFER,
            gl::DEPTH_STENCIL_ATTACHMENT,
            gl::RENDERBUFFER,
            rbo,
        );

        if gl.CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
            log(format!("Failed to setup Framebuffer"));
        }
        gl.BindFramebuffer(gl::FRAMEBUFFER, 0);

        Gl {
            gl,
            font_shader,
            line_shader,
            poly_shader,
            tex_shader,
            texture,
            xres: xres as f32,
            yres: yres as f32,
            fbo,
            colorbuf,
            window_scale,
        }
    }
}

impl Gl {
    pub fn clear_frame(&self) {
        unsafe {
            self.gl.Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn bind_framebuffer(&self) {
        unsafe {
            self.gl.Viewport(0, 0, self.xres as i32, self.yres as i32);
            self.gl.BindFramebuffer(gl::FRAMEBUFFER, self.fbo);
        }
    }

    pub fn unbind_framebuffer(&self) {
        unsafe {
            self.gl.Viewport(
                0,
                0,
                (self.xres * self.window_scale) as i32,
                (self.yres * self.window_scale) as i32,
            );
            self.gl.BindFramebuffer(gl::FRAMEBUFFER, 0);
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

            self.gl.Uniform1i(self.tex_shader.uniform_override_uv, 0);
            self.gl.Uniform2f(self.tex_shader.uniform_uv, 0.0, 0.0);
            self.gl.Uniform2f(self.tex_shader.uniform_duv, 1.0, 1.0);

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
        frame: bool,
        frame_u: f32,
        frame_v: f32,
        frame_du: f32,
        frame_dv: f32,
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

            self.gl
                .Uniform1i(self.tex_shader.uniform_override_uv, frame as i32);
            self.gl
                .Uniform2f(self.tex_shader.uniform_uv, frame_u, frame_v);
            self.gl
                .Uniform2f(self.tex_shader.uniform_duv, frame_du, frame_dv);

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

    pub fn draw_screen_billboard(&self, vao: u32, color: super::ecs::Color) {
        self.tex_shader.use_program(&self.gl);

        unsafe {
            self.gl.ActiveTexture(gl::TEXTURE0);
            self.gl.BindTexture(gl::TEXTURE_2D, self.colorbuf);
            self.gl.Uniform1i(self.tex_shader.uniform_tex_sampler, 0);
            self.gl.Uniform4f(
                self.tex_shader.uniform_color,
                color.r,
                color.g,
                color.b,
                color.a,
            );

            self.gl.Uniform1i(self.tex_shader.uniform_override_uv, 0);
            self.gl.Uniform2f(self.tex_shader.uniform_uv, 0.0, 0.0);
            self.gl.Uniform2f(self.tex_shader.uniform_duv, 0.0, 0.0);

            self.gl.BindVertexArray(vao);

            let mvp: Matrix4<f32> = Matrix4::identity();

            /*let mat = Matrix4::from_translation(Vector3::new(self.xres * 0.5, self.yres * 0.5, 0.0));
            let mvp = mvp * mat;

            let mat = Matrix4::from_angle_z(cgmath::Rad(0.0));
            let mvp = mvp * mat;

            let mat = Matrix4::from_nonuniform_scale(self.xres, -self.yres, 1.0);
            let mvp = mvp * mat;*/

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

    pub fn draw_tilemap(
        &self,
        x: f32,
        y: f32,
        angle: f32,
        sx: f32,
        sy: f32,
        vao: u32,
        count: usize,
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

            self.gl.Uniform1i(self.tex_shader.uniform_override_uv, 0);
            self.gl.Uniform2f(self.tex_shader.uniform_uv, 0.0, 0.0);
            self.gl.Uniform2f(self.tex_shader.uniform_duv, 1.0, 1.0);

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

            self.gl.DrawArrays(gl::TRIANGLES, 0, (count * 6) as i32);

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
        perspective: bool,
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
            let mut mvp = self.get_mvp();

            if perspective {

                mvp = Matrix4::new(
                    1.0, 0.0, 0.0, 0.0,
                    0.0, 1.0, 0.0, 0.0,
                    0.0, 0.0, 1.0, 0.0,
                    0.0, 0.0, 0.0, 1.0
                );


                let mat = cgmath::perspective(cgmath::Deg(45.0), 640.0 / 384.0, 0.001, 10.0);
                mvp = mvp * mat;

                let mat = Matrix4::from_translation(Vector3::new(0.0, -0.01, -0.04));
                mvp = mvp * mat;

                let mat = Matrix4::from_angle_y(cgmath::Rad(angle));
                mvp = mvp * mat;

                let mat = Matrix4::from_translation(Vector3::new(x, 0.0, y));
                mvp = mvp * mat;

                self.gl.Uniform1i(self.line_shader.uniform_alt, 1);

            }
            else {

                let mat = Matrix4::from_translation(Vector3::new(x, y, 0.0));
                mvp = mvp * mat;
                
                let mat = Matrix4::from_angle_z(cgmath::Rad(angle));
                mvp = mvp * mat;

                let mat = Matrix4::from_nonuniform_scale(sx, sy, 1.0);
                mvp = mvp * mat;

                self.gl.Uniform1i(self.line_shader.uniform_alt, 0);
            }            

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
