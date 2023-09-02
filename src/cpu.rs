mod instructions;
mod registers;

use std::ops::{BitAnd, BitOr, BitXor};

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
            Instruction::ADDr(target) => self.add_target(target, false),
            Instruction::ADDi(value) => self.add_value(value, false),
            Instruction::ADCr(target) => self.add_target(target, true),
            Instruction::ADCi(value) => self.add_value(value, true),
            Instruction::SUBr(target) => self.sub_target(target, false),
            Instruction::SUBi(value) => self.sub_value(value, false),
            Instruction::SBCr(target) => self.sub_target(target, true),
            Instruction::SBCi(value) => self.sub_value(value, true),
            Instruction::ANDr(target) => self.and_target(target),
            Instruction::ANDi(value) => self.and_value(value),
            Instruction::XORr(target) => self.xor_target(target),
            Instruction::XORi(value) => self.xor_value(value),
            Instruction::ORr(target) => self.or_target(target),
            Instruction::ORi(value) => self.or_value(value),
            Instruction::CPr(target) => self.cp_target(target),
            Instruction::CPi(value) => self.cp_value(value),
            _ => todo!(),
        }
    }

    fn add_target(&mut self, target: ArithmeticTarget, with_carry: bool) {
        self.add_value(self.read_target(target), with_carry);
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

    fn sub_target(&mut self, target: ArithmeticTarget, with_carry: bool) {
        self.sub_value(self.read_target(target), with_carry);
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

    fn and_target(&mut self, target: ArithmeticTarget) {
        self.and_value(self.read_target(target));
    }

    fn and_value(&mut self, value: u8) {
        let new_value = self.registers.a.bitand(value);
        self.registers.a = new_value;
    }

    fn xor_target(&mut self, target: ArithmeticTarget) {
        self.xor_value(self.read_target(target));
    }

    fn xor_value(&mut self, value: u8) {
        let new_value = self.registers.a.bitxor(value);
        self.registers.a = new_value;
    }

    fn or_target(&mut self, target: ArithmeticTarget) {
        self.or_value(self.read_target(target));
    }

    fn or_value(&mut self, value: u8) {
        let new_value = self.registers.a.bitor(value);
        self.registers.a = new_value;
    }

    fn cp_target(&mut self, target: ArithmeticTarget) {
        self.cp_value(self.read_target(target));
    }

    fn cp_value(&mut self, value: u8) {
        self.sub(value);
    }

    fn read_target(&self, target: ArithmeticTarget) -> u8 {
        let mut value = 0;
        match target {
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
}
