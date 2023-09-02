pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: FlagsRegister,
    pub h: u8,
    pub l: u8,
}

impl Registers {
    // AF
    pub fn get_af(&self) -> u16 {
        (self.a as u16) << 8 | u16::from(u8::from(self.f))
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.f = FlagsRegister::from((value & 0x00FF) as u8)
    }

    // BC
    pub fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0x00FF) as u8;
    }

    // DE
    pub fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0x00FF) as u8;
    }

    // HL
    pub fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0x00FF) as u8;
    }

    // Other
    pub fn set_flags(&mut self, value: u8, overflow: bool) {
        self.f.set(value, overflow, self.a);
    }

    pub fn set_carry_flag(&mut self) {
        self.f.set_carry_flag();
    }

    pub fn complement_carry_flag(&mut self) {
        self.f.complement_carry_flag();
    }

    pub fn cpl(&mut self) {
        self.f.cpl();
    }
}

#[derive(Clone, Copy)]
pub struct FlagsRegister {
    pub zero: bool,       // Z
    pub subtract: bool,   // N
    pub half_carry: bool, // H
    pub carry: bool,      // C
}

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

impl FlagsRegister {
    pub fn set(&mut self, value: u8, overflow: bool, a: u8) {
        self.zero = value == 0;
        self.subtract = false;
        self.carry = overflow;
        self.half_carry = (a & 0xF) + (value & 0xF) > 0xF;
    }

    pub fn set_carry_flag(&mut self) {
        self.zero = true;
        self.subtract = false;
        self.carry = true;
    }

    pub fn complement_carry_flag(&mut self) {
        self.zero = true;
        self.subtract = false;
        self.carry = !self.carry;
    }

    pub fn cpl(&mut self) {
        self.subtract = true;
        self.half_carry = true;
    }
}

impl std::convert::From<FlagsRegister> for u8 {
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION
            | (if flag.subtract { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POSITION
            | (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION
            | (if flag.carry { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
    }
}

impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        let zero = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0;
        let subtract = ((byte >> SUBTRACT_FLAG_BYTE_POSITION) & 0b1) != 0;
        let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;
        let carry = ((byte >> CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;

        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry,
        }
    }
}
