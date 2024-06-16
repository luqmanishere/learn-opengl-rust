use std::{ffi::CString, mem, ptr};

use anyhow::Result;
use gl::{
    types::{GLchar, GLfloat, GLint, GLsizei, GLsizeiptr, GLuint},
    ARRAY_BUFFER,
};
use glfw::{fail_on_errors, Action, Context, GlfwReceiver, Key, WindowHint, WindowMode};

pub fn main_1_3_1() -> Result<()> {
    const VERTEX_SHADER_SOURCE: &str = r"
        #version 330 core
        layout (location = 0) in vec3 aPos;

        void main ()
        {
            gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
        }
    ";

    const FRAGMENT_SHADER_SOURCE: &str = r"
        #version 330 core
        out vec4 FragColor;

        uniform vec4 ourColor;

        void main()
        {
            FragColor = ourColor;
        }
    ";

    let mut glfw = glfw::init(fail_on_errors!())?;

    glfw.window_hint(WindowHint::ContextVersionMajor(3));
    glfw.window_hint(WindowHint::ContextVersionMinor(3));
    glfw.window_hint(WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    if let Some((mut window, events)) =
        glfw.create_window(800, 600, "LearnOpenGL", WindowMode::Windowed)
    {
        window.make_current();
        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        // WARN: use f32 to avoid weirdness
        #[rustfmt::skip]
        let vertices: [f32; 9] = [
            -0.5, -0.5, 0.0,
             0.5, -0.5, 0.0,
             0.0,  0.5, 0.0
        ];

        // NOTE: compile vertex shaders
        let vertex_shader: GLuint;
        unsafe {
            vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            let c_str_vert = CString::new(VERTEX_SHADER_SOURCE.as_bytes()).unwrap();
            gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), ptr::null());
            gl::CompileShader(vertex_shader);
        }

        // FIXME: error is detected, but no log
        let mut success = gl::FALSE as GLint;
        let mut infolog = Vec::with_capacity(512);
        unsafe {
            gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(
                    vertex_shader,
                    512,
                    ptr::null_mut(),
                    infolog.as_mut_ptr() as *mut GLchar,
                );
                println!(
                    "ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}",
                    std::str::from_utf8(&infolog)?
                );
            }
        }

        // fragment shader
        let fragment_shader: GLuint;
        unsafe {
            fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            let c_str_frag = CString::new(FRAGMENT_SHADER_SOURCE.as_bytes()).unwrap();
            gl::ShaderSource(fragment_shader, 1, &c_str_frag.as_ptr(), ptr::null());
            gl::CompileShader(fragment_shader);
            gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(
                    fragment_shader,
                    512,
                    ptr::null_mut(),
                    infolog.as_mut_ptr() as *mut GLchar,
                );
                println!(
                    "ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}",
                    std::str::from_utf8(&infolog).unwrap()
                );
            }
        }

        // shader program
        let shader_program: GLuint;
        unsafe {
            shader_program = gl::CreateProgram();
            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, fragment_shader);
            gl::LinkProgram(shader_program);
            gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetProgramInfoLog(
                    shader_program,
                    512,
                    ptr::null_mut(),
                    infolog.as_mut_ptr() as *mut GLchar,
                );
                println!(
                    "ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}",
                    std::str::from_utf8(&infolog).unwrap()
                );
            }
            gl::UseProgram(shader_program);
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
        }

        let mut vbo: GLuint = 0;
        let mut vao: GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                ARRAY_BUFFER,
                (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                // TODO: revisit this
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                3 * mem::size_of::<GLfloat>() as GLsizei,
                ptr::null(),
            );
            gl::EnableVertexAttribArray(0);
        }

        while !window.should_close() {
            // all events including input
            process_events(&mut window, &events);

            // NOTE: start rendering process here
            unsafe {
                gl::ClearColor(0.2, 0.3, 0.3, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);

                gl::UseProgram(shader_program);
                let time_value = glfw.get_time();
                let green_value = (time_value.sin() / 2.0) + 0.5;
                let our_color = CString::new("ourColor")?;
                let vertex_color_location: GLint =
                    gl::GetUniformLocation(shader_program, our_color.as_ptr());
                gl::Uniform4f(vertex_color_location, 0.0, green_value as f32, 0.0, 1.0);
                gl::BindVertexArray(vao);
                gl::DrawArrays(gl::TRIANGLES, 0, 3);
            }
            window.swap_buffers();
            glfw.poll_events();
        }
    }
    Ok(())
}

fn process_events(window: &mut glfw::Window, events: &GlfwReceiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true)
            }
            _ => {}
        }
    }
}
