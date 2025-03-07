#ifndef __CPU
#define __CPU

#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>
#include "types.h"

typedef uint64_t u64;
typedef size_t usize;
typedef uint8_t u8;

typedef struct Flags {
    bool sf;
    bool zf;
    bool of;
} Flags;

typedef struct RegFile{
    u64 rax;
    u64 rcx;
    u64 rdx;
    u64 rbx;
    u64 rsp;
    u64 rbp;
    u64 rsi;
    u64 rdi;
    u64 r8;
    u64 r9;
    u64 r10;
    u64 r11;
    u64 r12;
    u64 r13;
    u64 r14;
} RegFile;

void set_reg(RegFile* file, Register index, u64 value);
u64 get_reg(RegFile* file, Register index);

typedef struct CPU {
    Flags flags;
    RegFile registers;
    u8 memory[0x400];
    usize program_counter;
    // begin ifun
    Cond condition;
    bool cnd;
    Op op;
    // end ifun
    Y86 curr;
    Register rA;
    Register rB;
    u64 valA;
    u64 valB;
    u64 valC;
    u64 valE;
    u64 valM;
    usize valP;
    bool stat;
} CPU;

CPU default_cpu();
void fetch(CPU* cpu);
void decode(CPU* cpu);
void execute(CPU* cpu);
void memory(CPU* cpu);
void writeback(CPU* cpu);
void program_counter(CPU* cpu);

#endif
