mod instructions;
mod registers;

use std::ops::{BitAnd, BitOr, BitXor, Not};

use crate::cpu::instructions::{ArithmeticTarget, Instruction};
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

    fn dec_register(&mut self, reg: ArithmeticTarget) {
        self.sub_register(reg, false);
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

    fn read_register(&self, reg: ArithmeticTarget) -> u8 {
        let mut value = 0;
        match reg {
            ArithmeticTarget::A => {
                value = self.registers.a;
            }
            ArithmeticTarget::B => {
                value = self.registers.b;
            }
            ArithmeticTarget::C => {
                value = self.registers.c;
            }
            ArithmeticTarget::D => {
                value = self.registers.d;
            }
            ArithmeticTarget::E => {
                value = self.registers.e;
            }
            ArithmeticTarget::H => {
                value = self.registers.h;
            }
            ArithmeticTarget::L => {
                value = self.registers.l;
            }
        }
        value
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
