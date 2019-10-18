mod emulator;
mod cpu;
mod graphics;
mod memory;

use emulator::Emulator;
use std::time::{SystemTime, Duration};

fn main() {

    let mut emu = Emulator::new();
    let timestep = Duration::from_secs(1) / 60;

    while emu.running {
        let begin = SystemTime::now();
        // Run the current cycle
        emu.update();
        // Wait until 1/60 of a second has passed
        while SystemTime::now() < begin + timestep { }
    }

}