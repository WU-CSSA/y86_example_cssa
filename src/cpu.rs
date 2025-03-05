use std::{hint::unreachable_unchecked, ops::{Index, IndexMut}, u64, ops::{Sub, Add, BitAnd, BitXor}};

use crate::isa::{Cond, Op, Register, Y86, Y86BLANK};

#[derive(Default)]
pub struct Flags {
    sf: bool,
    zf: bool,
    of: bool,
}

#[derive(Default)]
pub struct RegFile {
    rax: u64,
    rcx: u64,
    rdx: u64,
    rbx: u64,
    rsp: u64,
    rbp: u64,
    rsi: u64,
    rdi: u64,
    r8: u64,
    r9: u64,
    r10: u64,
    r11: u64,
    r12: u64,
    r13: u64,
    r14: u64,
}

impl Index<Register> for RegFile {
    type Output = u64;
    fn index(&self, index: Register) -> &Self::Output {
        match index {
            Register::RAX => &self.rax,
            Register::RCX => &self.rcx,
            Register::RDX => &self.rdx,
            Register::RBX => &self.rbx,
            Register::RSP => &self.rsp,
            Register::RBP => &self.rbp,
            Register::RSI => &self.rsi,
            Register::RDI => &self.rdi,
            Register::R8 => &self.r8,
            Register::R9 => &self.r9,
            Register::R10 => &self.r10,
            Register::R11 => &self.r11,
            Register::R12 => &self.r12,
            Register::R13 => &self.r13,
            Register::R14 => &self.r14,
        }
    }
}

impl IndexMut<Register> for RegFile {
    fn index_mut(&mut self, index: Register) -> &mut Self::Output {
        match index {
            Register::RAX => &mut self.rax,
            Register::RCX => &mut self.rcx,
            Register::RDX => &mut self.rdx,
            Register::RBX => &mut self.rbx,
            Register::RSP => &mut self.rsp,
            Register::RBP => &mut self.rbp,
            Register::RSI => &mut self.rsi,
            Register::RDI => &mut self.rdi,
            Register::R8 => &mut self.r8,
            Register::R9 => &mut self.r9,
            Register::R10 => &mut self.r10,
            Register::R11 => &mut self.r11,
            Register::R12 => &mut self.r12,
            Register::R13 => &mut self.r13,
            Register::R14 => &mut self.r14,
        }
    }
}

#[allow(non_snake_case)]
pub struct CPU {
    flags: Flags,
    registers: RegFile,
    memory: [u8; 0x400],
    program_counter: usize,
    // begin ifun
    condition: Cond,
    cnd: bool,
    op: Op,
    // end ifun
    curr: Y86BLANK,
    rA: Register,
    rB: Register,
    valA: u64,
    valB: u64,
    valC: u64,
    valE: u64,
    valM: u64,
    valP: usize,
    pub(super) stat: bool,
}

impl Default for CPU {
    fn default() -> Self {
        CPU {
            memory: [0; 0x400],
            flags: Default::default(),
            registers: Default::default(),
            program_counter: Default::default(),
            condition: Default::default(),
            cnd: Default::default(),
            op: Default::default(),
            curr: Default::default(),
            rA: Default::default(),
            rB: Default::default(),
            valA: Default::default(),
            valB: Default::default(),
            valC: Default::default(),
            valE: Default::default(),
            valM: Default::default(),
            valP: Default::default(),
            stat: Default::default(),
        }
    }
}

pub trait FeDeExMemWBPC {
    fn fetch(&mut self);
    fn decode(&mut self);
    fn execute(&mut self);
    fn memory(&mut self);
    fn writeback(&mut self);
    fn program_counter(&mut self);
}

impl FeDeExMemWBPC for CPU {
    fn fetch(&mut self) {
        let first = self.memory[self.program_counter];
        match first & 0xf0 {
            0x00 => {
                self.valP = self.program_counter + 1;
                self.curr = Y86BLANK::HALT;
            },
            0x10 => {
                self.valP = self.program_counter + 1;
                self.curr = Y86BLANK::NOOP;
            },
            0x20 => { 
                self.condition = (first & 0x0f).into();
                let regs = self.memory[self.program_counter + 1];
                self.rA = ((regs & 0xf0) >> 4).into();
                self.rB = (regs & 0x0f).into();

                self.valP = self.program_counter + 2;
                self.curr = Y86BLANK::CMOV;
            }
            0x30 => {
                let regs = self.memory[self.program_counter + 1];
                self.rB = (regs & 0x0f).into();

                let mut res: u64 = 0;
                for i in 0..8 {
                    res += (self.memory[self.program_counter + 2 + i] << (8 * i)) as u64;
                }

                self.valC = res;

                self.valP = self.program_counter + 10;
                self.curr = Y86BLANK::IRMOVQ;
            }
            0x40 => {
                let regs = self.memory[self.program_counter + 1];
                self.rA = ((regs & 0xf0) >> 4).into();
                self.rB = (regs & 0x0f).into();

                let mut res: u64 = 0;
                for i in 0..8 {
                    res += (self.memory[self.program_counter + 2 + i] << (8 * i)) as u64;
                }

                self.valC = res;

                self.valP = self.program_counter + 10;
                self.curr = Y86BLANK::RMMOVQ;
            }
            0x50 => {
                let regs = self.memory[self.program_counter + 1];
                self.rA = ((regs & 0xf0) >> 4).into();
                self.rB = (regs & 0x0f).into();

                let mut res: u64 = 0;
                for i in 0..8 {
                    res += (self.memory[self.program_counter + 2 + i] << (8 * i)) as u64;
                }

                self.valC = res;

                self.valP = self.program_counter + 10;
                self.curr = Y86BLANK::MRMOVQ;
            }
            0x60 => { 
                self.op = (first & 0x0f).into();

                let regs = self.memory[self.program_counter + 1];
                self.rA = ((regs & 0xf0) >> 4).into();
                self.rB = (regs & 0x0f).into();

                self.curr = Y86BLANK::OPQ;
            }
            0x70 => {
                self.condition = (first & 0x0f).into();

                let mut res: u64 = 0;
                for i in 0..8 {
                    res += (self.memory[self.program_counter + 1 + i] << (8 * i)) as u64;
                }
                self.valC = res;

                self.valP = self.program_counter + 9;
                self.curr = Y86BLANK::J;
            }
            0x80 => {
                let mut res: u64 = 0;
                for i in 0..8 {
                    res += (self.memory[self.program_counter + 1 + i] << (8 * i)) as u64;
                }
                self.valC = res;

                self.valP = self.program_counter + 9;
                self.curr = Y86BLANK::CALL;
            }
            0x90 => {
                self.valP = self.program_counter + 1;
                self.curr = Y86BLANK::RET;
            },
            0xa0 => {
                let regs = self.memory[self.program_counter + 1];
                self.rA = ((regs & 0xf0) >> 4).into();
                
                self.valP = self.program_counter + 2;
                self.curr = Y86BLANK::PUSHQ;
            }
            0xb0 => { 
                let regs = self.memory[self.program_counter + 1];
                self.rA = ((regs & 0xf0) >> 4).into();
                
                self.valP = self.program_counter + 2;
                self.curr = Y86BLANK::POPQ;
            }
            _ => unsafe { unreachable_unchecked() }
        }
    }

    fn decode(&mut self) {
        match &self.curr {
            Y86BLANK::CMOV => {
                self.valA = self.registers[self.rA];
            }
            Y86BLANK::RMMOVQ => {
                self.valA = self.registers[self.rA];
                self.valB = self.registers[self.rB];
            }
            Y86BLANK::MRMOVQ => {
                self.valB = self.registers[self.rB];
            }
            Y86BLANK::OPQ => {
                self.valA = self.registers[self.rA];
                self.valB = self.registers[self.rB];
            }
            Y86BLANK::CALL => {
                self.valB = self.registers[Register::RSP];
            }
            Y86BLANK::RET => {
                self.valA = self.registers[Register::RSP];
                self.valB = self.registers[Register::RSP];
            }
            Y86BLANK::PUSHQ => {
                self.valA = self.registers[self.rA];
                self.valB = self.registers[Register::RSP];
            }
            Y86BLANK::POPQ => {
                self.valA = self.registers[Register::RSP];
                self.valB = self.registers[Register::RSP];
            }
            _ => (),
        }
    }

    fn execute(&mut self) {
        match &self.curr {
            Y86BLANK::HALT => self.stat = false,
            // Condition handled in fetch
            Y86BLANK::CMOV => self.valE = self.valA,
            Y86BLANK::IRMOVQ => self.valE = self.valC,
            Y86BLANK::RMMOVQ | Y86BLANK::MRMOVQ => self.valE = self.valB + self.valC,
            Y86BLANK::OPQ => { 
                self.valE = [u64::wrapping_add, u64::wrapping_sub, u64::bitand, u64::bitxor][self.op as usize](self.valA, self.valB);
                match self.op {
                    Op::Add => self.flags.of = self.valE < self.valA || self.valE < self.valA,
                    Op::Sub => self.flags.of = self.valE > self.valA || self.valE > self.valA,
                    _ => (),
                }
                if self.valE == 0 {
                    self.flags.zf = true;
                }
                if self.valE & 1 << 63 != 0 {
                    self.flags.sf = true;
                }
            }
            Y86BLANK::J => {
                let (sf, zf, of) = (self.flags.sf, self.flags.zf, self.flags.of);
                self.cnd = match self.condition {
                    Cond::None => true,
                    Cond::Le   => sf || zf,
                    Cond::L    => sf && zf,
                    Cond::E    => zf,
                    Cond::Ne   => !zf,
                    Cond::Ge   => !sf || zf,
                    Cond::G    => !sf && !zf,
                };
            }
            // Condition handled in fetch for [`Y86BLANK::J`]
            Y86BLANK::CALL => self.valE = self.valB - 8,
            Y86BLANK::RET => self.valE = self.valB + 8,
            Y86BLANK::PUSHQ => self.valE = self.valB - 8,
            Y86BLANK::POPQ => self.valE = self.valB + 8,
            _ => (),
        }
    }

    fn memory(&mut self) {
        match &self.curr {
            Y86BLANK::RMMOVQ => { 
                for i in 0..8 {
                    self.memory[self.valE as usize + i] = ((self.valA >> (8 * i)) & 0xff) as u8;
                }
            }
            Y86BLANK::MRMOVQ => {
                let mut res = 0;
                for i in 0..8 {
                    res += (self.memory[self.valE as usize + i] << (8 * i)) as u64;
                }
                self.valM = res;
            }
            Y86BLANK::CALL => {
                for i in 0..8 {
                    self.memory[self.valE as usize + i] = ((self.valP >> (8 * i)) & 0xff) as u8;
                }
            }
            Y86BLANK::RET => {
                let mut res = 0;
                for i in 0..8 {
                    res += (self.memory[self.valA as usize + i] << (8 * i)) as u64;
                }
                self.valM = res;
            }
            Y86BLANK::PUSHQ => {
                for i in 0..8 {
                    self.memory[self.valE as usize + i] = ((self.valA >> (8 * i)) & 0xff) as u8;
                }
            }
            Y86BLANK::POPQ => {
                let mut res = 0;
                for i in 0..8 {
                    res += (self.memory[self.valA as usize + i] << (8 * i)) as u64;
                }
                self.valM = res;
            }
            _ => (),
        }
    }

    fn writeback(&mut self) {
        match &self.curr {
            Y86BLANK::CMOV   => if self.cnd { self.registers[self.rB] = self.valE },
            Y86BLANK::IRMOVQ => self.registers[self.rB] = self.valE,
            Y86BLANK::MRMOVQ => self.registers[self.rA] = self.valM,
            Y86BLANK::OPQ    => self.registers[self.rB] = self.valE,

            Y86BLANK::CALL   => self.registers[Register::RSP] = self.valE,
            Y86BLANK::RET    => self.registers[Register::RSP] = self.valE,
            Y86BLANK::PUSHQ  => self.registers[Register::RSP] = self.valE,
            Y86BLANK::POPQ   => {
                self.registers[Register::RSP] = self.valE;
                self.registers[self.rA] = self.valM;
            },
            _ => (),
        }
    }

    fn program_counter(&mut self) {
        match &self.curr {
            Y86BLANK::HALT   => self.program_counter = 0,
            Y86BLANK::NOOP   => self.program_counter = self.valP,
            Y86BLANK::CMOV   => self.program_counter = self.valP,
            Y86BLANK::IRMOVQ => self.program_counter = self.valP,
            Y86BLANK::RMMOVQ => self.program_counter = self.valP,
            Y86BLANK::MRMOVQ => self.program_counter = self.valP,
            Y86BLANK::OPQ    => self.program_counter = self.valP,
            Y86BLANK::J      => self.program_counter = if (self.cnd) { self.valC as usize } else { self.valP },
            Y86BLANK::CALL   => self.program_counter = self.valC as usize,
            Y86BLANK::RET    => self.program_counter = self.valM as usize,
            Y86BLANK::PUSHQ  => self.program_counter = self.valP,
            Y86BLANK::POPQ   => self.program_counter = self.valP,
        }
    }
}
