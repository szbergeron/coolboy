#[macro_use]
extern crate bitflags;

mod cartridge;
mod cpu;
mod emulator;
mod memory;

use emulator::Emulator;

fn main() {
    let mut emu = Emulator::from_file("roms/tetris.gb");

    emu.run();
}
