pub struct Memory {
    memory: [u8; 0xFFFF]
}

impl Memory {

    pub fn new() -> Memory {
        Memory {
            memory: [0; 0xFFFF]
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

}