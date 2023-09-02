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

    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        new_value
    }
}
