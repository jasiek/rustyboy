mod cpu;

use crate::cpu::new_cpu;

fn main() {
    let mut cpu = new_cpu();
    cpu.test_run();

    println!("Hello, world!");
}
