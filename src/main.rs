use glutin::event::{Event, WindowEvent, ElementState, VirtualKeyCode};
use glutin::event_loop::{EventLoop, ControlFlow};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use gl::types::*;
use std::ffi::CString;
use std::ptr;

mod graphics;
mod audio;
mod input;
mod game_loader;

fn main() {
    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new().with_title("Sony PlayStation 1 Emulator");
    let context = ContextBuilder::new().build_windowed(window_builder, &event_loop).unwrap();
    let window = context.window();

    context.make_current().unwrap();
    graphics::initialize();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent { event: WindowEvent::KeyboardInput { input, .. }, .. } => {
                if let Some(VirtualKeyCode::Escape) = input.virtual_keycode {
                    *control_flow = ControlFlow::Exit;
                }
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                graphics::render();
            }
            _ => (),
        }
    });
}