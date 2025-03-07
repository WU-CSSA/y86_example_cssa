#ifndef __TYPES
#define __TYPES

typedef enum Register {
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
} Register;

typedef enum Cond {
    None = 0x00,
    Le   = 0x01,
    L    = 0x02,
    E    = 0x03,
    Ne   = 0x04,
    Ge   = 0x05,
    G    = 0x06,
} Cond;

typedef enum Op {
    Add = 0x00,
    Sub = 0x01,
    And = 0x02,
    Xor = 0x03,
} Op;

typedef enum Y86 {
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
} Y86;

#endif
