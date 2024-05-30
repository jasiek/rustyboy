mod helpers;

use helpers::*;

use rustyboy::cpu::instructions::{ArithmeticTarget8, Instruction};
use rustyboy::cpu::new_cpu;

#[test]
fn test_add() {
    let mut cpu = new_cpu();
    stream_json_from_array_file("sm83-test-data/alu_tests/v1/add.json", |result| {
        cpu.reset();
        cpu.execute(Instruction::LDri(ArithmeticTarget8::A, x_as(&result)));
        cpu.execute(Instruction::LDri(ArithmeticTarget8::B, y_as(&result)));
        cpu.execute(Instruction::ADDr(ArithmeticTarget8::B));

        assert_eq!(result_value::<u8>(&result), cpu.registers.a);
    })
}
