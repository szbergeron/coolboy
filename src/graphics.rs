extern crate minifb;

use super::memory::Memory;

use minifb::{Key, Window, WindowOptions};

pub struct Screen {
    window: Window,
    buffer: Vec<u32>
}

impl Screen {

    pub fn new() -> Screen {
        Screen {
            buffer: vec![0; 160 * 144],
            window: Window::new("Test", 160, 144, WindowOptions::default()).unwrap_or_else(|_| { panic!("{}"); })
        }
    }

    pub fn display(&mut self, memory: &Memory) {
        for i in self.buffer.iter_mut() {
            *i = 0
        }
        self.window.update_with_buffer(&self.buffer).unwrap();
    }

}