use std::fs::File;
use std::io;
use std::io::Read;

pub struct Cartridge {
    data: Vec<u8>,
}

impl Cartridge {
    pub fn from_file(filename: &str) -> Result<Cartridge, io::Error> {
        let mut file = File::open(filename).unwrap();
        let mut buffer = Vec::with_capacity(0x200_000);

        file.read(&mut buffer)?;
        Ok(Cartridge { data: buffer })
    }

    pub fn empty() -> Cartridge {
        Cartridge { data: Vec::new() }
    }
}
