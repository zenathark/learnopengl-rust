use gl;
use gl::types::*;
use glfw;
use glfw::Context;
use glfw::{Action, Key};
use std::sync::mpsc::Receiver;

use std::ffi::CString;
use std::mem;
use std::os::raw::c_void;
use std::ptr;
use std::str;

const HEIGHT: u32 = 600;
const WIDTH: u32 = 800;

pub fn main0101() {
    // Instantiate a GLFW window
    let mut ctx = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    ctx.window_hint(glfw::WindowHint::ContextVersion(4, 1));
    ctx.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    // Create a window object
    let (mut window, events) = ctx
        .create_window(WIDTH, HEIGHT, "LearnOpenGL", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW Window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // gl: load all OpenGL function pointers
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    // game loop
    while !window.should_close() {
        // events
        process_input0101(&mut window, &events);
        // swap buffers and pool IO
        window.swap_buffers();
        ctx.poll_events();
    }
}

fn process_input0101(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            // resize viewport if needed
            glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                gl::Viewport(0, 0, width, height);
            },
            // Catch esc for window closing
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true)
            }
            _ => {}
        }
    }
}

pub fn main0102() {
    // Instantiate a GLFW window
    let mut ctx = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    ctx.window_hint(glfw::WindowHint::ContextVersion(4, 1));
    ctx.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    // Create a window object
    let (mut window, events) = ctx
        .create_window(WIDTH, HEIGHT, "LearnOpenGL", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW Window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // gl: load all OpenGL function pointers
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    // game loop
    while !window.should_close() {
        // events
        process_input0101(&mut window, &events);
        // render
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        // swap buffers and pool IO
        window.swap_buffers();
        ctx.poll_events();
    }
}

pub fn main0201() {
    // Instantiate a GLFW window
    println!("Creating window");
    let mut ctx = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    ctx.window_hint(glfw::WindowHint::ContextVersion(4, 1));
    ctx.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    // Create a window object
    let (mut window, events) = ctx
        .create_window(
            WIDTH,
            HEIGHT,
            "LearnOpenGL Chapter 1.2.1",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create a GLFW Window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    //--------------------------------------------------------------------------
    // Create shaders
    let (shader_program, vao) = unsafe {
        //----------------------------VERTEX-------------------------------------
        println!("Loading vertex shader");
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        // Using CString for ffi with opengl
        let vertex_source =
            CString::new(include_str!("triangle.vert")).expect("Reading vertex failed");
        gl::ShaderSource(vertex_shader, 1, &vertex_source.as_ptr(), ptr::null());
        gl::CompileShader(vertex_shader);
        let mut success = i32::from(gl::FALSE) as GLint;
        let mut info_log = Vec::with_capacity(512);
        info_log.set_len(512 - 1);
        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
        if success != i32::from(gl::TRUE) as GLint {
            gl::GetShaderInfoLog(
                vertex_shader,
                512,
                ptr::null_mut(),
                info_log.as_mut_ptr() as *mut GLchar,
            );
            panic!(str::from_utf8(&info_log).unwrap().to_string());
        }

        //---------------------------FRAGMENT-----------------------------------
        println!("Loading fragment shader");
        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        let fragment_source =
            CString::new(include_str!("triangle.frag")).expect("Reading fragment failed");
        gl::ShaderSource(
            fragment_shader,
            1,
            &fragment_source.as_ptr() as *const _,
            ptr::null(),
        );
        gl::CompileShader(fragment_shader);
        let mut success = i32::from(gl::FALSE) as GLint;
        let mut info_log = Vec::with_capacity(512);
        info_log.set_len(512 - 1);
        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
        if success != i32::from(gl::TRUE) as GLint {
            gl::GetShaderInfoLog(
                vertex_shader,
                512,
                ptr::null_mut(),
                info_log.as_mut_ptr() as *mut GLchar,
            );
            panic!(str::from_utf8(&info_log).unwrap().to_string());
        }
        //-------------------------SHADER PROGRAM-------------------------------
        println!("Linking program");
        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);
        let mut success = i32::from(gl::FALSE) as GLint;
        let mut info_log = Vec::with_capacity(512);
        info_log.set_len(512 - 1);
        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
        if success != i32::from(gl::TRUE) as GLint {
            gl::GetProgramInfoLog(
                shader_program,
                512,
                ptr::null_mut(),
                info_log.as_mut_ptr() as *mut GLchar,
            );
            panic!(str::from_utf8(&info_log).unwrap().to_string());
        }
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
        //----------------------Vertex Data-------------------------------------

        let vertices: [f32; 9] = [
            -0.5, -0.5, 0.0, // left
            0.5, -0.5, 0.0, // right
            0.0, 0.5, 0.0, // top
        ];
        println!("Linking vertex array");
        let mut vao_ptr = vec![0; 1];
        gl::GenVertexArrays(1 as GLsizei, vao_ptr.as_mut_ptr() as *mut GLuint);
        let vao = vao_ptr[0];
        let mut vbo_ptr = vec![0; 1];
        gl::GenBuffers(1 as GLsizei, vbo_ptr.as_mut_ptr() as *mut GLuint);
        let vbo = vbo_ptr[0];
        gl::BindVertexArray(vao as GLuint);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * mem::size_of::<f32>()) as GLsizeiptr,
            &vertices[0] as *const f32 as *const c_void,
            gl::STATIC_DRAW,
        );
        println!("Creating vertex attribute");
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * mem::size_of::<GLfloat>() as GLsizei,
            ptr::null(),
        );
        gl::EnableVertexAttribArray(0);
        println!("Enabling vertex attribute");
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
        (shader_program, vao)
    };

    //game loop
    println!("Starting game loop");
    while !window.should_close() {
        process_input0101(&mut window, &events);
        // render
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::UseProgram(shader_program);
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        // swap buffers and pool IO
        window.swap_buffers();
        ctx.poll_events();
    }
}
