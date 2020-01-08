use super::memory::Memory;
use super::registers::Registers;

pub struct Cpu {
    registers: Registers,
    pc: u16,
    sp: u16,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            registers: Registers::new(),
            pc: 0,
            sp: 0,
        }
    }

    pub fn execute(&self, memory: &mut Memory) -> i32 {
        // unimplemented!()
        return 0;
    }
}
