use std::{
    ffi::{c_void, CString},
    mem, ptr,
};

use anyhow::{Context, Result};
use gl::{
    types::{GLfloat, GLint, GLsizei, GLsizeiptr, GLuint},
    ARRAY_BUFFER,
};
use glfw::{
    fail_on_errors, Action, Context as GLContext, GlfwReceiver, Key, WindowHint, WindowMode,
};

use crate::shaders::Shader;

pub fn main_1_4_2() -> Result<()> {
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
        let vertices: [f32; 32] = [
             // positions     // colors       // texture coords
             0.5,  0.5, 0.0,  1.0, 0.0, 0.0,  1.0, 1.0, // top right
             0.5, -0.5, 0.0,  0.0, 1.0, 0.0,  1.0, 0.0, // bottom right
            -0.5, -0.5, 0.0,  0.0, 0.0, 1.0,  0.0, 0.0, // bottom left
            -0.5,  0.5, 0.0,  1.0, 1.0, 0.0,  0.0, 1.0, // top left
        ];

        let indices = [
            0, 1, 3, // first triangle
            1, 2, 3, // second triangle
        ];

        let shader = Shader::new(
            "src/_1_getting_started/shaders/4.2.texture_uniform.vs",
            "src/_1_getting_started/shaders/4.2.texture_uniform.fs",
        )?;

        let mut vbo: GLuint = 0;
        let mut vao: GLuint = 0;
        let mut ebo: GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                ARRAY_BUFFER,
                (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                // TODO: revisit this
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &indices[0] as *const i32 as *const c_void,
                gl::STATIC_DRAW,
            );

            let stride = 8 * mem::size_of::<GLfloat>() as GLsizei;

            // position
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
            gl::EnableVertexAttribArray(0);

            // color
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                stride,
                (3 * mem::size_of::<GLfloat>()) as *const c_void,
            );
            gl::EnableVertexAttribArray(1);

            // texture coords
            gl::VertexAttribPointer(
                2,
                2,
                gl::FLOAT,
                gl::FALSE,
                stride,
                (6 * mem::size_of::<GLfloat>()) as *const c_void,
            );
            gl::EnableVertexAttribArray(2);
        }

        let image1 = image::open("resources/textures/container.jpg").context("opening texture")?;
        let image1_data = image1.as_bytes().to_vec();
        let image2 = image::open("resources/textures/awesomeface.png")?.flipv();
        let image2_data = image2.as_bytes().to_vec();

        // load textures
        let mut texture1: GLuint = 0;
        let mut texture2: GLuint = 0;
        unsafe {
            // texture 1
            gl::GenTextures(1, &mut texture1);
            gl::BindTexture(gl::TEXTURE_2D, texture1); // work on this texture
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint); // wrap texture with GL_REPEAT
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
            // set texture filtering parameters
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR_MIPMAP_LINEAR as GLint,
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as GLint,
                image1.width() as GLint,
                image1.height() as GLint,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                // instead of doing image_data[0], use .as_ptr()
                image1_data.as_ptr() as *const c_void,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);

            // texture 2
            gl::GenTextures(1, &mut texture2);
            gl::BindTexture(gl::TEXTURE_2D, texture2); // work on this texture
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint); // wrap texture with GL_REPEAT
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
            // set texture filtering parameters
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR_MIPMAP_LINEAR as GLint,
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as GLint,
                image2.width() as GLint,
                image2.height() as GLint,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                image2_data.as_ptr() as *const c_void,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);

            shader.use_program();
            shader.set_int(&CString::new("texture1")?, 0);
            shader.set_int(&CString::new("texture2")?, 1);
        }

        while !window.should_close() {
            // all events including input
            process_events(&mut window, &events);

            // NOTE: start rendering process here
            unsafe {
                gl::ClearColor(0.2, 0.3, 0.3, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);

                shader.use_program();

                gl::ActiveTexture(gl::TEXTURE0);
                gl::BindTexture(gl::TEXTURE_2D, texture1);
                gl::ActiveTexture(gl::TEXTURE1);
                gl::BindTexture(gl::TEXTURE_2D, texture2);

                gl::BindVertexArray(vao);
                gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
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
