mod instructions;
mod registers;

use std::ops::{BitAnd, BitOr, BitXor, Not};

use crate::cpu::instructions::{ArithmeticTarget, ArithmeticTarget16, Instruction};
use crate::cpu::registers::{FlagsRegister, Registers};

pub struct CPU {
    registers: Registers,
    pc: u16,
}

pub fn new_cpu() -> CPU {
    CPU {
        registers: Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: FlagsRegister {
                zero: false,
                subtract: false,
                half_carry: false,
                carry: false,
            },
            h: 0,
            l: 0,
            sp: 0, // TODO: 02/09/2023 (jps): this is probably the wrong value
        },
        pc: 0,
    }
}

impl CPU {
    pub fn test_run(&mut self) {
        self.execute(Instruction::ADDr(ArithmeticTarget::A));
        self.execute(Instruction::ADDi(3));
        self.execute(Instruction::ADCr(ArithmeticTarget::B));
        self.execute(Instruction::ADCi(2));
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            // // 8-bit Load instructions
            Instruction::LDrr(dest_reg, src_reg) => self.ld_rr(dest_reg, src_reg),
            Instruction::LDri(dest_reg, value) => self.ld_ri(dest_reg, value),

            // 8 bit arithmetic / logic
            Instruction::ADDr(reg) => self.add_register(reg, false),
            Instruction::ADDi(value) => self.add_value(value, false),
            Instruction::ADCr(reg) => self.add_register(reg, true),
            Instruction::ADCi(value) => self.add_value(value, true),
            Instruction::SUBr(reg) => self.sub_register(reg, false),
            Instruction::SUBi(value) => self.sub_value(value, false),
            Instruction::SBCr(reg) => self.sub_register(reg, true),
            Instruction::SBCi(value) => self.sub_value(value, true),
            Instruction::ANDr(reg) => self.and_register(reg),
            Instruction::ANDi(value) => self.and_value(value),
            Instruction::XORr(reg) => self.xor_register(reg),
            Instruction::XORi(value) => self.xor_value(value),
            Instruction::ORr(reg) => self.or_register(reg),
            Instruction::ORi(value) => self.or_value(value),
            Instruction::CPr(reg) => self.cp_register(reg),
            Instruction::CPi(value) => self.cp_value(value),
            Instruction::INCr(reg) => self.inc_register(reg),
            Instruction::DECr(reg) => self.dec_register(reg),
            Instruction::CPL => self.cpl(),

            /* 16-bit Arithmetic/Logic instructions */
            Instruction::ADDHLRR(reg16) => self.add_hl_rr(reg16),
            Instruction::INCRR(reg16) => self.inc_register16(reg16),
            Instruction::DECRR(reg16) => self.dec_register16(reg16),

            /* Rotate and Shift instructions */
            Instruction::RLCA => self.rotate_a_left(false),
            Instruction::RLA => self.rotate_a_left(true),
            Instruction::RRCA => self.rotate_a_right(false),
            Instruction::RRA => self.rotate_a_right(true),
            Instruction::RLCr(reg8) => self.rotate_r_left(reg8, false),
            Instruction::RRCr(reg8) => self.rotate_r_right(reg8, false),
            Instruction::RLr(reg8) => self.rotate_r_left(reg8, true),
            Instruction::RRr(reg8) => self.rotate_r_right(reg8, true),

            // CPU Control instructions
            Instruction::SCF => self.set_carry_flag(),
            Instruction::CCF => self.complement_carry_flag(),
            Instruction::NOP => {}
            Instruction::HALT => { /* todo later */ }
        }
    }

    fn add_register(&mut self, reg: ArithmeticTarget, with_carry: bool) {
        self.add_value(self.read_register(reg), with_carry);
    }

    fn add_value(&mut self, value: u8, with_carry: bool) {
        let mut new_value = self.add(value);
        self.registers.a = new_value;
        if with_carry && self.registers.f.carry {
            new_value = self.add(1);
            self.registers.a = new_value;
        }
    }

    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        self.registers.set_flags(new_value, did_overflow);

        new_value
    }

    fn sub_register(&mut self, reg: ArithmeticTarget, with_carry: bool) {
        self.sub_value(self.read_register(reg), with_carry);
    }

    fn sub_value(&mut self, value: u8, with_carry: bool) {
        let mut new_value = self.sub(value);
        self.registers.a = new_value;
        if with_carry && self.registers.f.carry {
            new_value = self.sub(1);
            self.registers.a = new_value;
        }
    }

    fn sub(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_sub(value);
        self.registers.set_flags(new_value, did_overflow);

        new_value
    }

    fn and_register(&mut self, reg: ArithmeticTarget) {
        self.and_value(self.read_register(reg));
    }

    fn and_value(&mut self, value: u8) {
        let new_value = self.registers.a.bitand(value);
        self.registers.a = new_value;
    }

    fn xor_register(&mut self, reg: ArithmeticTarget) {
        self.xor_value(self.read_register(reg));
    }

    fn xor_value(&mut self, value: u8) {
        let new_value = self.registers.a.bitxor(value);
        self.registers.a = new_value;
    }

    fn or_register(&mut self, reg: ArithmeticTarget) {
        self.or_value(self.read_register(reg));
    }

    fn or_value(&mut self, value: u8) {
        let new_value = self.registers.a.bitor(value);
        self.registers.a = new_value;
    }

    fn cp_register(&mut self, reg: ArithmeticTarget) {
        self.cp_value(self.read_register(reg));
    }

    fn cp_value(&mut self, value: u8) {
        self.sub(value);
    }

    fn inc_register(&mut self, reg: ArithmeticTarget) {
        self.add_register(reg, false);
    }

    fn inc_register16(&mut self, reg: ArithmeticTarget16) {
        self.write_register16(reg, self.read_register16(reg) + 1)
    }

    fn dec_register(&mut self, reg: ArithmeticTarget) {
        self.sub_register(reg, false);
    }

    fn dec_register16(&mut self, reg: ArithmeticTarget16) {
        self.write_register16(reg, self.read_register16(reg) - 1)
    }

    fn set_carry_flag(&mut self) {
        self.registers.set_carry_flag();
    }

    fn complement_carry_flag(&mut self) {
        self.registers.complement_carry_flag();
    }

    fn ld_rr(&mut self, dest_reg: ArithmeticTarget, src_reg: ArithmeticTarget) {
        self.ld_ri(dest_reg, self.read_register(src_reg));
    }

    fn ld_ri(&mut self, dest_reg: ArithmeticTarget, value: u8) {
        self.write_register(dest_reg, value);
    }

    fn cpl(&mut self) {
        self.registers.a = self.registers.a.not();
        self.registers.cpl();
    }

    fn add_hl_rr(&mut self, src_reg: ArithmeticTarget16) {
        let (new_value, _overflow) = self
            .registers
            .get_hl()
            .overflowing_add(self.read_register16(src_reg));
        // TODO: 02/09/2023 (jps): does this belong here?
        let [_, l] = new_value.to_be_bytes();
        // TODO: 02/09/2023 (jps): not sure this is correct
        self.registers.set_flags(l, false);
    }

    fn rotate_r_left(&mut self, reg: ArithmeticTarget, with_carry: bool) {
        let mut val = self.read_register(reg);
        if with_carry {
            let chop = val & 1;
            val = val << 1;
            if self.registers.f.carry {
                val = val & 1
            }
            self.registers.f.carry = chop == 1;
        } else {
            val = val.rotate_left(1);
        }
        self.write_register(reg, val);
    }

    fn rotate_r_right(&mut self, reg: ArithmeticTarget, with_carry: bool) {
        let mut val = self.read_register(reg);
        if with_carry {
            let chop = val & 1;
            val = val >> 1;
            if self.registers.f.carry {
                val = val | (1 << 7)
            }
            self.registers.f.carry = chop == 1;
        } else {
            val = val.rotate_right(1);
        }
        self.write_register(reg, val);
    }

    fn rotate_a_left(&mut self, with_carry: bool) {
        self.rotate_r_left(ArithmeticTarget::A, with_carry);
    }

    fn rotate_a_right(&mut self, with_carry: bool) {
        self.rotate_r_right(ArithmeticTarget::A, with_carry);
    }

    fn read_register16(&self, reg: ArithmeticTarget16) -> u16 {
        match reg {
            ArithmeticTarget16::BC => self.registers.get_bc(),
            ArithmeticTarget16::DE => self.registers.get_de(),
            ArithmeticTarget16::HL => self.registers.get_hl(),
            ArithmeticTarget16::SP => self.registers.sp,
        }
    }

    fn write_register16(&mut self, reg: ArithmeticTarget16, value: u16) {
        match reg {
            ArithmeticTarget16::BC => self.registers.set_bc(value),
            ArithmeticTarget16::DE => self.registers.set_de(value),
            ArithmeticTarget16::HL => self.registers.set_hl(value),
            ArithmeticTarget16::SP => self.registers.sp = value,
        }
    }

    fn read_register(&self, reg: ArithmeticTarget) -> u8 {
        match reg {
            ArithmeticTarget::A => self.registers.a,
            ArithmeticTarget::B => self.registers.b,
            ArithmeticTarget::C => self.registers.c,
            ArithmeticTarget::D => self.registers.d,
            ArithmeticTarget::E => self.registers.e,
            ArithmeticTarget::H => self.registers.h,
            ArithmeticTarget::L => self.registers.l,
        }
    }

    fn write_register(&mut self, reg: ArithmeticTarget, value: u8) {
        match reg {
            ArithmeticTarget::A => {
                self.registers.a = value;
            }
            ArithmeticTarget::B => {
                self.registers.b = value;
            }
            ArithmeticTarget::C => {
                self.registers.c = value;
            }
            ArithmeticTarget::D => {
                self.registers.d = value;
            }
            ArithmeticTarget::E => {
                self.registers.e = value;
            }
            ArithmeticTarget::H => {
                self.registers.h = value;
            }
            ArithmeticTarget::L => {
                self.registers.l = value;
            }
        }
    }
}
