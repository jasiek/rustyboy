mod instructions;
mod registers;

use crate::cpu::instructions::{Instruction, ArithmeticTarget};
use crate::cpu::registers::{Registers, FlagsRegister};

pub struct CPU {
    registers: Registers,
    pc: u16,
}

pub fn NewCPU() -> CPU {
    CPU{
        registers: Registers{
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: FlagsRegister{
                zero: false,
                subtract: false,
                half_carry: false,
                carry: false
            },
            h: 0,
            l: 0
        },
        pc: 0
    }
}

impl CPU {
    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(target) => match target {
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.add(value);
                    self.registers.a = new_value;
                },
                _ => todo!()
            },
            _ => todo!()
        }
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
