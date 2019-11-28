use super::cartridge::Cartridge;
use super::cpu::Cpu;
use super::memory::Memory;

use std::time::{Duration, SystemTime};

pub struct Emulator {
    cartridge: Cartridge,
    cpu: Cpu,
    memory: Memory,
}

impl Emulator {
    pub fn from_file(filename: &str) -> Emulator {
        Emulator {
            cartridge: Cartridge::from_file(filename),
            cpu: Cpu::new(),
            memory: Memory::new(),
        }
    }

    pub fn run(&mut self) {
        let timestep = Duration::from_secs(1) / 60;

        loop {
            let begin = SystemTime::now();
            // Run the current cycle
            self.update();
            // Wait until 1/60 of a second has passed
            while SystemTime::now() < begin + timestep {}
        }
    }

    fn update(&mut self) {
        let mut cycles = 0;

        // 69905 CPU cycles per frame
        while cycles < 69905 {
            cycles += self.cpu.execute(&mut self.memory);
        }
    }
}
