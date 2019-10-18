use super::cpu::CPU;
use super::memory::Memory;
use super::graphics::Screen;

pub struct Emulator {
    pub cpu: CPU,
    pub screen: Screen,
    pub memory: Memory,
    pub running: bool
}

impl Emulator {

    pub fn new() -> Emulator {
        Emulator {
            cpu: CPU::new(),
            screen: Screen::new(),
            memory: Memory::new(),
            running: true
        }
    }

    fn load(&mut self, filename: &str) {
        
    }

    pub fn update(&mut self) {
        // GB can execute a maximum of 4,194,304 cycles a second
        // This is the number of cycles per frame (@ 60 fps)
        const MAX_CYCLES: u32 = 69905;
        let mut cycles = 0;

        while cycles < MAX_CYCLES {
            // Execute one CPU cycle
            self.cpu.step(&mut self.memory);
            cycles += 1;
        }

        // Display graphics for the current frame
        self.screen.display(&self.memory)
    }

}