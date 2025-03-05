#![allow(non_snake_case, unused)]

use cpu::{CPU, FeDeExMemWBPC};
pub mod isa;
pub mod cpu;

impl CPU {
    fn run() {
        let mut cpu = CPU::default();
        loop {
            cpu.fetch();
            cpu.decode();
            cpu.execute();
            cpu.memory();
            cpu.writeback();
            cpu.program_counter();
            if !cpu.stat {
                break;
            }
        }
    }
}
