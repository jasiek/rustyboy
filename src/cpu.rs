mod instructions;
mod registers;

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
    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(target) => self.add_target(target),
            Instruction::ADDHL(target) => self.add_hl_target(target),
            _ => todo!(),
        }
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

    fn add_target(&mut self, target: ArithmeticTarget) {
        let new_value = self.add(self.read_target(target));
        self.registers.a = new_value;
    }

    fn add_hl_target(&mut self, target: ArithmeticTarget) {
        let new_value = self.add_hl(self.read_target(target));
        self.registers.set_hl(new_value);
    }

    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        self.set_flags(new_value, did_overflow);

        new_value
    }

    fn add_hl(&mut self, value: u8) -> u16 {
        let (new_hl_value, did_overflow) =
            self.registers.get_hl().overflowing_add(u16::from(value));
        let [h, l] = new_hl_value.to_be_bytes();
        self.set_flags(l, did_overflow);

        new_hl_value
    }

    fn set_flags(&mut self, value: u8, overflow: bool) {
        self.registers.f.zero = value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = overflow;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
    }
}
