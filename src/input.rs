use glutin::event::{ElementState, VirtualKeyCode};

pub struct InputHandler {
    keys: [bool; 256],
}

impl InputHandler {
    pub fn new() -> Self {
        InputHandler { keys: [false; 256] }
    }

    pub fn key_event(&mut self, key: VirtualKeyCode, state: ElementState) {
        let index = key as usize;
        self.keys[index] = state == ElementState::Pressed;
    }

    pub fn is_key_pressed(&self, key: VirtualKeyCode) -> bool {
        let index = key as usize;
        self.keys[index]
    }
}