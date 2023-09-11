mod helpers;

use serde_json::Deserializer;
use std::fs::File;

use helpers::*;

use rustyboy::cpu::instructions::{ArithmeticTarget8, Instruction};
use rustyboy::cpu::new_cpu;

#[test]
fn test_add() {
    let file = File::open("sm83-test-data/alu_tests/v1/add.json").unwrap();
    let stream = Deserializer::from_reader(file).into_iter::<TestEntry>();
    for value in stream {
        let mut cpu = new_cpu();
        let entry = value.unwrap();
        cpu.execute(Instruction::LDri(ArithmeticTarget8::A, x_as(&entry)));
        cpu.execute(Instruction::LDri(ArithmeticTarget8::B, y_as(&entry)));
        cpu.execute(Instruction::ADDr(ArithmeticTarget8::B));

        assert_eq!(result_value::<u8>(&entry), cpu.registers.a);
    }
}
