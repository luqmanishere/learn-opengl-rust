use std::{ffi::c_void, mem, ptr};

use anyhow::Result;
use gl::{types::*, ARRAY_BUFFER};
use glfw::{fail_on_errors, Action, Context, GlfwReceiver, Key, WindowHint, WindowMode};

use crate::shaders::Shader;

pub fn main_1_3_3() -> Result<()> {
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
        let vertices: [f32; 18] = [
            // position          colors
            -0.5, -0.5, 0.0, 1.0, 0.0, 0.0, // bottom right
             0.5, -0.5, 0.0, 0.0, 1.0, 0.0, // bottom left
            0.0,  0.5, 0.0, 0.0, 0.0, 1.0, // top
        ];

        // NOTE: compile vertex shaders
        let shader = Shader::new(
            "src/_1_getting_started/shaders/3.3.shader.vs",
            "src/_1_getting_started/shaders/3.3.shader.fs",
        )?;

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
            let stride = 6 * mem::size_of::<GLfloat>() as GLsizei;
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                6 * mem::size_of::<GLfloat>() as GLsizei,
                ptr::null(),
            );
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                stride,
                (3 * mem::size_of::<GLfloat>()) as *const c_void,
            );
            // dont forget this like me else your triangle black af
            gl::EnableVertexAttribArray(1);
        }

        while !window.should_close() {
            // all events including input
            process_events(&mut window, &events);

            // INFO: start rendering process here
            unsafe {
                gl::ClearColor(0.2, 0.3, 0.3, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);

                shader.use_program();
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
