use std::{
    ffi::{CStr, CString},
    fs::File,
    io::Read,
    ptr, str,
};

use anyhow::{Context, Result};
use cgmath::{Array, Matrix, Matrix4, Vector3};
use gl::types::*;

pub struct Shader {
    pub id: GLuint,
}

#[allow(dead_code)]
impl Shader {
    pub fn new(vertex_path: &str, fragment_path: &str) -> Result<Shader> {
        let mut shader = Shader { id: 0 };
        // read shader code from system
        let mut vshader_file = File::open(vertex_path).context("vertex shader path")?;
        let mut fshader_file = File::open(fragment_path)?;

        let mut vertex_code = String::new();
        vshader_file.read_to_string(&mut vertex_code)?;
        let mut fragment_code = String::new();
        fshader_file.read_to_string(&mut fragment_code)?;

        let vertex_code = CString::new(vertex_code.as_bytes())?;
        let fragment_code = CString::new(fragment_code.as_bytes())?;

        // compile shader code
        unsafe {
            let vertex = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex, 1, &vertex_code.as_ptr(), ptr::null());
            gl::CompileShader(vertex);
            shader.check_compile_errors(vertex, "VERTEX")?;
            // fragment Shader
            let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragment, 1, &fragment_code.as_ptr(), ptr::null());
            gl::CompileShader(fragment);
            shader.check_compile_errors(fragment, "FRAGMENT")?;
            // shader Program
            let id = gl::CreateProgram();
            gl::AttachShader(id, vertex);
            gl::AttachShader(id, fragment);
            gl::LinkProgram(id);
            shader.check_compile_errors(id, "PROGRAM")?;
            // delete the shaders as they're linked into our program now and no longer necessary
            gl::DeleteShader(vertex);
            gl::DeleteShader(fragment);
            shader.id = id;
        }
        Ok(shader)
    }

    /// activate the shader
    /// ------------------------------------------------------------------------
    pub unsafe fn use_program(&self) {
        gl::UseProgram(self.id)
    }

    /// utility uniform functions
    /// ------------------------------------------------------------------------
    pub unsafe fn set_bool(&self, name: &CStr, value: bool) {
        gl::Uniform1i(gl::GetUniformLocation(self.id, name.as_ptr()), value as i32);
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn set_int(&self, name: &CStr, value: i32) {
        gl::Uniform1i(gl::GetUniformLocation(self.id, name.as_ptr()), value);
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn set_float(&self, name: &CStr, value: f32) {
        gl::Uniform1f(gl::GetUniformLocation(self.id, name.as_ptr()), value);
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn set_vector3(&self, name: &CStr, value: &Vector3<f32>) {
        gl::Uniform3fv(
            gl::GetUniformLocation(self.id, name.as_ptr()),
            1,
            value.as_ptr(),
        );
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn set_vec3(&self, name: &CStr, x: f32, y: f32, z: f32) {
        gl::Uniform3f(gl::GetUniformLocation(self.id, name.as_ptr()), x, y, z);
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn set_mat4(&self, name: &CStr, mat: &Matrix4<f32>) {
        gl::UniformMatrix4fv(
            gl::GetUniformLocation(self.id, name.as_ptr()),
            1,
            gl::FALSE,
            mat.as_ptr(),
        );
    }

    /// utility function for checking shader compilation/linking errors.
    /// ------------------------------------------------------------------------
    unsafe fn check_compile_errors(&self, shader: u32, type_: &str) -> Result<()> {
        let mut success = gl::FALSE as GLint;

        // the buffer code came from http://nercury.github.io/rust/opengl/tutorial/2018/02/10/opengl-in-rust-from-scratch-03-compiling-shaders.html
        if type_ != "PROGRAM" {
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                // get log length
                let mut len: GLint = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);

                // allocate buffer of correct size
                let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
                // fill it with len spaces
                buffer.extend([b' '].iter().cycle().take(len as usize));
                // convert buffer to CString
                let error: CString = unsafe { CString::from_vec_unchecked(buffer) };

                gl::GetShaderInfoLog(shader, len, ptr::null_mut(), error.as_ptr() as *mut GLchar);
                println!(
                    "ERROR::SHADER_COMPILATION_ERROR of type: {} | length: {}\n{}\n \
                          -- --------------------------------------------------- -- ",
                    type_,
                    len,
                    error.into_string()?
                );
            }
        } else {
            gl::GetProgramiv(shader, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                // get log length
                let mut len: GLint = 0;
                gl::GetProgramiv(shader, gl::INFO_LOG_LENGTH, &mut len);

                // allocate buffer of correct size
                let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
                // fill it with len spaces
                buffer.extend([b' '].iter().cycle().take(len as usize));
                // convert buffer to CString
                let error: CString = unsafe { CString::from_vec_unchecked(buffer) };

                gl::GetProgramInfoLog(shader, len, ptr::null_mut(), error.as_ptr() as *mut GLchar);
                println!(
                    "ERROR::PROGRAM_LINKING_ERROR of type: {} | length: {}\n{}\n \
                          -- --------------------------------------------------- -- ",
                    type_,
                    len,
                    error.into_string()?
                );
            }
        }
        Ok(())
    }

    /// Only used in 4.9 Geometry shaders - ignore until then (shader.h in original C++)
    pub fn with_geometry_shader(
        vertex_path: &str,
        fragment_path: &str,
        geometry_path: &str,
    ) -> Result<Self> {
        let mut shader = Shader { id: 0 };
        // 1. retrieve the vertex/fragment source code from filesystem
        let mut v_shader_file =
            File::open(vertex_path).unwrap_or_else(|_| panic!("Failed to open {}", vertex_path));
        let mut f_shader_file = File::open(fragment_path)
            .unwrap_or_else(|_| panic!("Failed to open {}", fragment_path));
        let mut g_shader_file = File::open(geometry_path)
            .unwrap_or_else(|_| panic!("Failed to open {}", geometry_path));
        let mut vertex_code = String::new();
        let mut fragment_code = String::new();
        let mut geometry_code = String::new();
        v_shader_file
            .read_to_string(&mut vertex_code)
            .expect("Failed to read vertex shader");
        f_shader_file
            .read_to_string(&mut fragment_code)
            .expect("Failed to read fragment shader");
        g_shader_file
            .read_to_string(&mut geometry_code)
            .expect("Failed to read geometry shader");

        let v_shader_code = CString::new(vertex_code.as_bytes()).unwrap();
        let f_shader_code = CString::new(fragment_code.as_bytes()).unwrap();
        let g_shader_code = CString::new(geometry_code.as_bytes()).unwrap();

        // 2. compile shaders
        unsafe {
            // vertex shader
            let vertex = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex, 1, &v_shader_code.as_ptr(), ptr::null());
            gl::CompileShader(vertex);
            shader.check_compile_errors(vertex, "VERTEX")?;
            // fragment Shader
            let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragment, 1, &f_shader_code.as_ptr(), ptr::null());
            gl::CompileShader(fragment);
            shader.check_compile_errors(fragment, "FRAGMENT")?;
            // geometry shader
            let geometry = gl::CreateShader(gl::GEOMETRY_SHADER);
            gl::ShaderSource(geometry, 1, &g_shader_code.as_ptr(), ptr::null());
            gl::CompileShader(geometry);
            shader.check_compile_errors(geometry, "GEOMETRY")?;

            // shader Program
            let id = gl::CreateProgram();
            gl::AttachShader(id, vertex);
            gl::AttachShader(id, fragment);
            gl::AttachShader(id, geometry);
            gl::LinkProgram(id);
            shader.check_compile_errors(id, "PROGRAM")?;
            // delete the shaders as they're linked into our program now and no longer necessary
            gl::DeleteShader(vertex);
            gl::DeleteShader(fragment);
            gl::DeleteShader(geometry);
            shader.id = id;
        }

        Ok(shader)
    }
}
impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}
