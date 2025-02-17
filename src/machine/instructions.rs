pub enum OpR8 {
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
    A,
}

pub enum OpR16 {
    BC,
    DE,
    HL,
    SP,
}

pub enum OpR16Stk {
    BC,
    DE,
    HL,
    AF,
}

pub enum OpR16Mem {
    BC,
    DE,
    HLPlus,
    HLMinus,
}

pub enum OpCond {
    NZ,
    Z,
    NC,
    C,
}

pub enum Operand {
    R8(OpR8),
    R16(OpR16),
    R16Stk(OpR16Stk),
    R16Mem(OpR16Mem),
    Cond(OpCond),
    B3(u8),
    Tgt(u8),
    Imm8(u8),
    Imm16(u16),
}

pub enum Instruction {
    Adc(Operand),
    Add(Operand),
    And(Operand),
    Bit(Operand),
    CallCond(OpCond, Operand),
    Call(Operand),
    Ccf,
    Cp(Operand),
    Cpl,
    Daa,
    Dec(Operand),
    Di,
    Ei,
    Halt,
    Inc(Operand),
    Jp(Operand),
    JpCond(OpCond, Operand),
    Jr(Operand),
    JrCond(OpCond, Operand),
    Ld(Operand, Operand),
    Ldh(Operand, Operand),
    Nop,
    Or(Operand),
    Pop(Operand),
    Push(Operand),
    Res(Operand, Operand),
    RetCond(Operand),
    Ret,
    Reti,
    Rl(Operand),
    Rla,
    Rlc(Operand),
    Rlca,
    Rr(Operand),
    Rrca,
    Rst(Operand),
    Sbc(Operand),
    Scf,
    Set(Operand, Operand),
    Sla(Operand),
    Sra(Operand),
    Srl(Operand),
    Stop,
    Sub(Operand),
    Swap(Operand),
    Xor(Operand),
}
