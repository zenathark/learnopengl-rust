
use gl;
use glfw;
use glfw::Context;
use glfw::{Key, Action};
use std::sync::mpsc::Receiver;

const HEIGHT: u32 = 600;
const WIDTH: u32 = 800;

pub fn main0101() {
    // Instantiate a GLFW window
    let mut ctx = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    ctx.window_hint(glfw::WindowHint::ContextVersion(4, 1));
    ctx.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    // Create a window object
    let (mut window, events) = ctx.create_window(WIDTH, HEIGHT, "LearnOpenGL", glfw::WindowMode::Windowed)
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
            glfw::WindowEvent::FramebufferSize(width, height) => {
                unsafe { gl::Viewport(0, 0, width, height); }
            }
            // Catch esc for window closing
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
            _ => {}
        }
    }
}

pub fn main0102() {
    // Instantiate a GLFW window
    let mut ctx = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    ctx.window_hint(glfw::WindowHint::ContextVersion(4, 1));
    ctx.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    // Create a window object
    let (mut window, events) = ctx.create_window(WIDTH, HEIGHT, "LearnOpenGL", glfw::WindowMode::Windowed)
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
