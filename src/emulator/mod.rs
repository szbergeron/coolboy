mod cartridge;
mod cpu;
mod memory;
mod registers;

use cartridge::Cartridge;
use cpu::Cpu;
use memory::Memory;

use std::io;

use std::time::{Duration, SystemTime};

pub struct Emulator {
    cartridge: Cartridge,
    cpu: Cpu,
    memory: Memory,
}

impl Emulator {
    pub fn from_file(filename: &str) -> Result<Emulator, io::Error> {
        Ok(Emulator {
            cartridge: Cartridge::from_file(filename)?,
            cpu: Cpu::new(),
            memory: Memory::initialized(),
           })
    }

    pub fn run<F>(&mut self, update: F) 
        where F: FnMut(Self) {
        
    }

    fn update(&mut self) {
        let mut cycles = 0;

        // 69905 CPU cycles per frame
        while cycles < 69905 {
            cycles += self.cpu.execute(&mut self.memory);
        }
    }
}
