use super::memory::Memory;

bitflags! {
    struct Flags: u8 {
        const zero = 0b1000_0000;
        const subtract = 0b0100_0000;
        const half_carry = 0b0010_0000;
        const carry = 0b0001_0000;
    }
}

struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: Flags,
    h: u8,
    l: u8,
}

pub struct Cpu {
    registers: Registers,
}

impl std::convert::From<u8> for Flags {
    fn from(n: u8) -> Self { 
        Flags::from_bits_truncate(n)
    }
}

impl std::convert::From<Flags> for u8 {
    fn from(flags: Flags) -> Self {
        flags.bits
    }
}

impl Registers {

    pub fn new() -> Registers {
        Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: Flags::from(0),
            h: 0,
            l: 0,
        }
    }

    pub fn af(&self) -> u16 {
        (self.a as u16) << 8 |
        u8::from(self.f) as u16
    }

    pub fn bc(&self) -> u16 {
        (self.b as u16) << 8 |
        self.c as u16
    }

    pub fn de(&self) -> u16 {
        (self.d as u16) << 8 |
        self.e as u16
    }

    pub fn hl(&self) -> u16 {
        (self.h as u16) << 8 |
        self.l as u16
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = Flags::from((value & 0x00FF) as u8);
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = (value & 0x00FF) as u8;
    }

    pub fn set_d(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = (value & 0x00FF) as u8;
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = (value & 0x00FF) as u8;
    }

}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            registers: Registers::new()
        }
    }

    pub fn execute(&self, memory: &mut Memory) -> i32 {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {

    use super::Flags;

    #[test]
    fn flags_to_u8_individual() {
        assert_eq!(Flags::zero.bits, u8::from(Flags::zero));
        assert_eq!(Flags::subtract.bits, u8::from(Flags::subtract));
        assert_eq!(Flags::half_carry.bits, u8::from(Flags::half_carry));
        assert_eq!(Flags::carry.bits, u8::from(Flags::carry));
    }

    #[test]
    fn flags_to_u8_all() {
        let flags = Flags::zero | Flags::subtract | Flags::half_carry | Flags::carry;
        assert_eq!(flags.bits, u8::from(flags));
    }

    #[test]
    fn u8_to_flags_individual() {
        assert!(Flags::from(Flags::zero).contains(Flags::zero));
        assert!(Flags::from(Flags::subtract).contains(Flags::subtract));
        assert!(Flags::from(Flags::half_carry).contains(Flags::half_carry));
        assert!(Flags::from(Flags::carry).contains(Flags::carry));
    }

    #[test]
    fn u8_to_flags_all() {
        let flags = Flags::zero | Flags::subtract | Flags::half_carry | Flags::carry;
        assert!(Flags::from(flags).contains(Flags::zero));
        assert!(Flags::from(flags).contains(Flags::subtract));
        assert!(Flags::from(flags).contains(Flags::half_carry));
        assert!(Flags::from(flags).contains(Flags::carry));
    }

}