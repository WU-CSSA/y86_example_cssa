use std::{hint::unreachable_unchecked};

#[derive(Clone, Copy, Default)]
#[repr(u8)]
pub enum Register {
    #[default]
    RAX = 0x00,
    RCX = 0x01,
    RDX = 0x02,
    RBX = 0x03,
    RSP = 0x04,
    RBP = 0x05,
    RSI = 0x06,
    RDI = 0x07,
    R8  = 0x08,
    R9  = 0x09,
    R10 = 0x0a,
    R11 = 0x0b,
    R12 = 0x0c,
    R13 = 0x0d,
    R14 = 0x0e,
}

impl From<Register> for u8 {
    fn from(value: Register) -> Self {
        value as u8
    }
}

impl From<u8> for Register {
    fn from(value: u8) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

#[derive(Clone, Copy, Default)]
#[repr(u8)]
pub enum Cond {
    #[default]
    None = 0x00,
    Le   = 0x01,
    L    = 0x02,
    E    = 0x03,
    Ne   = 0x04,
    Ge   = 0x05,
    G    = 0x06,
}

impl From<Cond> for u8 {
    fn from(value: Cond) -> Self {
        value as u8
    }
}

impl From<u8> for Cond {
    fn from(value: u8) -> Self {
        match value {
            0 => Cond::None,
            1 => Cond::Le,
            2 => Cond::L,
            3 => Cond::E,
            4 => Cond::Ne,
            5 => Cond::Ge,
            6 => Cond::G,
            _ => unsafe { unreachable_unchecked() }
        }
    }
}

#[derive(Clone, Copy, Default)]
#[repr(u8)]
pub enum Op {
    #[default]
    Add = 0x00,
    Sub = 0x01,
    And = 0x02,
    Xor = 0x03,
}

impl From<Op> for u8 {
    fn from(value: Op) -> Self {
        value as u8
    }
}

impl From<u8> for Op {
    fn from(value: u8) -> Self {
        match value {
            0 => Op::Add,
            1 => Op::Sub,
            2 => Op::And,
            3 => Op::Xor,
            _ => unsafe { unreachable_unchecked() }
        }
    }
}

#[derive(Clone, Copy, Default)]
#[repr(u8)]
pub enum Y86BLANK {
    #[default]
    HALT   = 0x00,
    NOOP   = 0x10,
    CMOV   = 0x20,
    IRMOVQ = 0x30,
    RMMOVQ = 0x40,
    MRMOVQ = 0x50,
    OPQ    = 0x60,
    J      = 0x70,
    CALL   = 0x80,
    RET    = 0x90,
    PUSHQ  = 0xa0,
    POPQ   = 0xb0,
}

pub enum Y86 {
    HALT,
    NOOP,
    CMOV(Cond, Register, Register),
    IRMOVQ(Register, u64),
    RMMOVQ(Register, Register, u64),
    MRMOVQ(Register, Register, u64),
    OPQ(Op, Register, Register),
    J(Cond, u64),
    CALL(u64),
    RET,
    PUSHQ(Register),
    POPQ(Register),
}

impl From<Y86> for Vec<u8> {
    fn from(value: Y86) -> Self {
        #[allow(non_snake_case, unused)]
        match value {
            Y86::HALT => {
                vec![Y86BLANK::HALT as u8]
            }
            Y86::NOOP => {
                vec![Y86BLANK::NOOP as u8]
            }
            Y86::CMOV(cond, rA, rB) => {
                let cond: u8 = cond.into();
                vec![Y86BLANK::CMOV as u8 | cond, rA.into(), rB.into()]
            }
            Y86::IRMOVQ(rB, u64) => {
                let rB: u8 = rB.into();
                let arr: [u8; 8] = unsafe { std::mem::transmute(u64) };
                let mut init = vec![Y86BLANK::IRMOVQ as u8, 0xf0 | rB];
                init.append(&mut arr.into_iter().collect());
                init
            }
            Y86::RMMOVQ(rA, rB, u64) => {
                let rA: u8 = rA.into();
                let rB: u8 = rB.into();
                let arr: [u8; 8] = unsafe { std::mem::transmute(u64) };
                let mut init = vec![Y86BLANK::RMMOVQ as u8, rA << 4 | rB];
                init.append(&mut arr.into());
                init
            }
            Y86::MRMOVQ(rA, rB, u64) => {
                let rA: u8 = rA.into();
                let rB: u8 = rB.into();
                let arr: [u8; 8] = unsafe { std::mem::transmute(u64) };
                let mut init = vec![Y86BLANK::MRMOVQ as u8, rA << 4 | rB];
                init.append(&mut arr.into());
                init
            }
            Y86::OPQ(op, rA, rB) => {
                let rA: u8 = rA.into();
                let rB: u8 = rB.into();
                let op: u8 = op.into();
                vec![Y86BLANK::OPQ as u8 | op, rA << 4 | rB]
            }
            Y86::J(cond, u64) => {
                let cond: u8 = cond.into();
                let arr: [u8; 8] = unsafe { std::mem::transmute(u64) };
                let mut init = vec![Y86BLANK::J as u8 | cond];
                init.append(&mut arr.into());
                init
            }
            Y86::CALL(u64) => {
                let arr: [u8; 8] = unsafe { std::mem::transmute(u64) };
                let mut init = vec![Y86BLANK::CALL as u8];
                init.append(&mut arr.into());
                init
            }
            Y86::RET => {
                vec![Y86BLANK::RET as u8]
            }
            Y86::PUSHQ(rA) => {
                let rA: u8 = rA.into();
                vec![Y86BLANK::PUSHQ as u8, rA << 4 | 0x0f]
            }
            Y86::POPQ(rA) => {
                let rA: u8 = rA.into();
                vec![Y86BLANK::POPQ as u8, rA << 4 | 0x0f]
            }
        }
    }
}
