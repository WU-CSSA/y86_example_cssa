#include "cpu.h"

CPU default_cpu() {
    CPU cpu = {
        .flags = {
            .zf = false,
            .sf = false,
            .of = false,
        },
        .registers = {
            .rax = 0,
            .rcx = 0,
            .rdx = 0,
            .rbx = 0,
            .rsp = 0,
            .rbp = 0,
            .rsi = 0,
            .rdi = 0,
            .r8 = 0,
            .r9 = 0,
            .r10 = 0,
            .r11 = 0,
            .r12 = 0,
            .r13 = 0,
            .r14 = 0,
        },
        .memory = { 0 },
        .program_counter = 0,
        .condition = None,
        .cnd = false,
        .op = 0,
        .curr = 0,
        .rA   = 0,
        .rB   = 0,
        .valA = 0,
        .valB = 0,
        .valC = 0,
        .valE = 0,
        .valM = 0,
        .valP = 0,
        .stat = false,
    };
    return cpu;
}

u64 get_reg(RegFile *file, Register index) {
    return ((u64*)file)[index];
}

void set_reg(RegFile *file, Register index, u64 value) {
    ((u64*)file)[index] = value;
}

void fetch(CPU* cpu) {
    u8 op = cpu->memory[cpu->program_counter];
    u8 regs;
    u64 res;
    switch (op & 0xf0) {
        case 0x00:
            cpu->curr = HALT;
            cpu->valP = cpu->program_counter + 1;
            break;
        case 0x10:
            cpu->curr = NOOP;
            cpu->valP = cpu->program_counter + 1;
            break;
        case 0x20:
            cpu->curr = CMOV;
            cpu->condition = (op & 0x0f);
            regs = cpu->memory[cpu->program_counter + 1];
            cpu->rA = (regs & 0xf0) >> 4;
            cpu->rB = (regs & 0xf0);
            cpu->valP = cpu->program_counter + 2;
            break;
        case 0x30:
            cpu->curr = IRMOVQ;
            regs = cpu->memory[cpu->program_counter + 1];
            cpu->rB = (regs & 0xf0);

            res = *((u64*)(cpu->memory + cpu->program_counter + 2));
            cpu->valC = res;
            cpu->valP = cpu->program_counter + 10;
            break;
        case 0x40:
            cpu->curr = RMMOVQ;
            regs = cpu->memory[cpu->program_counter + 1];
            cpu->rA = (regs & 0xf0) >> 4;
            cpu->rB = (regs & 0xf0);
            res = *((u64*)(cpu->memory + cpu->program_counter + 2));
            cpu->valP = cpu->program_counter + 10;
            break;
        case 0x50:
            cpu->curr = MRMOVQ;
            regs = cpu->memory[cpu->program_counter + 1];
            cpu->rA = (regs & 0xf0) >> 4;
            cpu->rB = (regs & 0xf0);
            res = *((u64*)(cpu->memory + cpu->program_counter + 2));
            cpu->valP = cpu->program_counter + 10;
            break;
        case 0x60:
            cpu->curr = OPQ;
            cpu->op = op & 0x0f;
            regs = cpu->memory[cpu->program_counter + 1];
            cpu->rA = (regs & 0xf0) >> 4;
            cpu->rB = (regs & 0xf0);
            cpu->valP = cpu->program_counter + 2;
            break;
        case 0x70:
            cpu->curr = J;
            cpu->condition = (op & 0x0f);
            cpu->valC = *((u64*)(cpu->memory + cpu->program_counter + 1));
            cpu->valP = cpu->program_counter + 9;
            break;
        case 0x80:
            cpu->curr = CALL;
            cpu->valC = *((u64*)(cpu->memory + cpu->program_counter + 1));
            cpu->valP = cpu->program_counter + 9;
            break;
        case 0x90:
            cpu->curr = RET;
            cpu->valP = cpu->program_counter + 1;
            break;
        case 0xa0:
            cpu->curr = PUSHQ;
            regs = cpu->memory[cpu->program_counter + 1];
            cpu->rA = (regs & 0xf0) >> 4;
            cpu->rB = (regs & 0xf0);
            cpu->valP = cpu->program_counter + 2;
            break;
        case 0xb0:
            cpu->curr = POPQ;
            regs = cpu->memory[cpu->program_counter + 1];
            cpu->rA = (regs & 0xf0) >> 4;
            cpu->rB = (regs & 0xf0);
            cpu->valP = cpu->program_counter + 2;
            break;
    }
}

void decode(CPU* cpu) {
    switch (cpu->curr) {
        case CMOV:
            cpu->valA = get_reg(&cpu->registers, cpu->rA);
            break;
        case RMMOVQ:
            cpu->valA = get_reg(&cpu->registers, cpu->rA);
            cpu->valB = get_reg(&cpu->registers, cpu->rB);
            break;
        case MRMOVQ:
            cpu->valB = get_reg(&cpu->registers, cpu->rB);
            break;
        case OPQ:
            cpu->valA = get_reg(&cpu->registers, cpu->rA);
            cpu->valB = get_reg(&cpu->registers, cpu->rB);
            break;
        case CALL:
            cpu->valB = get_reg(&cpu->registers, RSP);
            break;
        case RET:
            cpu->valA = get_reg(&cpu->registers, RSP);
            cpu->valB = get_reg(&cpu->registers, RSP);
            break;
        case PUSHQ:
            cpu->valA = get_reg(&cpu->registers, cpu->rA);
            cpu->valB = get_reg(&cpu->registers, RSP);
            break;
        case POPQ:
            cpu->valA = get_reg(&cpu->registers, RSP);
            cpu->valB = get_reg(&cpu->registers, RSP);
        default:
            break;
    }
}

u64 add(u64 a, u64 b) {
    return a + b;
}

u64 sub(u64 a, u64 b) {
    return a - b;
}

u64 and(u64 a, u64 b) {
    return a & b;
}

u64 xor(u64 a, u64 b) {
    return a ^ b;
}

void execute(CPU* cpu) {
    u64 (*actions[4])(u64, u64) = {add, sub, and, xor};
    bool sf;
    bool zf;
    bool of;
    switch (cpu->curr) {
        case HALT:
            cpu->stat = false;
            break;
        case CMOV:
            cpu->valE = cpu->valA;
            break;
        case IRMOVQ:
            cpu->valE = cpu->valC;
            break;
        case RMMOVQ:
        case MRMOVQ:
            cpu->valE = cpu->valB + cpu->valC;
            break;
        case OPQ:
            cpu->valE = actions[cpu->op](cpu->valA, cpu->valB);
            if (cpu->op == Add)
                cpu->flags.of = cpu->valE < cpu->valA || cpu->valE < cpu->valB;
            else if (cpu->op == Sub)
                cpu->flags.of = cpu->valE > cpu->valA || cpu->valE > cpu->valB;

            if (cpu->valE == 0)
                cpu->flags.zf = true;
            if (cpu->valE & ((u64)1 << 63))
                cpu->flags.sf = true;
            break;
        case J:
            sf = cpu->flags.sf;
            zf = cpu->flags.zf;
            of = cpu->flags.of;
            switch (cpu->condition) {
                case None:
                    cpu->cnd = true;
                    break;
                case Le:
                    cpu->cnd = sf || zf;
                    break;
                case L:
                    cpu->cnd = sf && zf;
                    break;
                case E:
                    cpu->cnd = zf;
                    break;
                case Ne:
                    cpu->cnd = !zf;
                    break;
                case Ge:
                    cpu->cnd = !sf || zf;
                    break;
                case G:
                    cpu->cnd = !sf && !zf;
                    break;
            }
            break;
        case CALL:
            cpu->valE = cpu->valB - 8;
            break;
        case RET:
            cpu->valE = cpu->valB + 8;
            break;
        case PUSHQ:
            cpu->valE = cpu->valB - 8;
            break;
        case POPQ:
            cpu->valE = cpu->valB + 8;
            break;
        default:
            break;
    }
}

void memory(CPU* cpu) {
    switch (cpu->curr) {
        case RMMOVQ:
            *((u64*)(cpu->memory + cpu->valE)) = cpu->valA;
            break;
        case MRMOVQ:
            cpu->valM = *((u64*)(cpu->memory + cpu->valE));
            break;
        case CALL:
            *((u64*)(cpu->memory + cpu->valE)) = cpu->valA;
            break;
        case RET:
            cpu->valM = *((u64*)(cpu->memory + cpu->valA));
            break;
        case PUSHQ:
            *((u64*)(cpu->memory + cpu->valE)) = cpu->valA;
            break;
        case POPQ:
            cpu->valM = *((u64*)(cpu->memory + cpu->valA));
            break;
        default:
            break;
    }
}

void writeback(CPU* cpu) {
    switch (cpu->curr) {
        case CMOV:
            if (cpu->cnd)
                set_reg(&cpu->registers, cpu->rB, cpu->valE);
            break;
        case IRMOVQ:
            set_reg(&cpu->registers, cpu->rB, cpu->valE);
            break;
        case MRMOVQ:
            set_reg(&cpu->registers, cpu->rA, cpu->valM);
            break;
        case OPQ:
            set_reg(&cpu->registers, cpu->rB, cpu->valE);
            break;
        case CALL:
            set_reg(&cpu->registers, RSP, cpu->valE);
            break;
        case RET:
            set_reg(&cpu->registers, RSP, cpu->valE);
            break;
        case PUSHQ:
            set_reg(&cpu->registers, RSP, cpu->valE);
            break;
        case POPQ:
            set_reg(&cpu->registers, RSP, cpu->valE);
            set_reg(&cpu->registers, cpu->rA, cpu->valM);
            break;
        default:
            break;
    }
}

void program_counter(CPU* cpu) {
    switch (cpu->curr) {
        case HALT:
            cpu->program_counter = 0;
            break;
        case NOOP:
            cpu->program_counter = cpu->valP;
            break;
        case CMOV:
            cpu->program_counter = cpu->valP;
            break;
        case IRMOVQ:
            cpu->program_counter = cpu->valP;
            break;
        case RMMOVQ:
            cpu->program_counter = cpu->valP;
            break;
        case MRMOVQ:
            cpu->program_counter = cpu->valP;
            break;
        case OPQ:
            cpu->program_counter = cpu->valP;
            break;
        case J:
            if (cpu->cnd)
                cpu->program_counter = cpu->valC;
            else
                cpu->program_counter = cpu->valP;
            break;
        case CALL:
            cpu->program_counter = cpu->valC;
            break;
        case RET:
            cpu->program_counter = cpu->valM;
            break;
        case PUSHQ:
            cpu->program_counter = cpu->valP;
            break;
        case POPQ:
            cpu->program_counter = cpu->valP;
            break;
        default:
            break;
    }
}

int main() {
    CPU cpu = default_cpu();
    while (true) {
        fetch(&cpu);
        decode(&cpu);
        execute(&cpu);
        memory(&cpu);
        writeback(&cpu);
        program_counter(&cpu);
        if (!cpu.stat)
            break;
    }
}
