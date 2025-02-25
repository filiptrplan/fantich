/// All the types are derived from this page:
/// https://gbdev.io/pandocs/CPU_Instruction_Set.html#cpu-instruction-set

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
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

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum OpR16 {
    BC,
    DE,
    HL,
    SP,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum OpR16Stk {
    BC,
    DE,
    HL,
    AF,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum OpR16Mem {
    BC,
    DE,
    HLPlus,
    HLMinus,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum OpCond {
    NZ,
    Z,
    NC,
    C,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Operand {
    R8(OpR8),
    R16(OpR16),
    R16Stk(OpR16Stk),
    R16Mem(OpR16Mem),
    Cond(OpCond),
    /// 3-bit index
    B3(u8),
    /// `rst`'s target address, divided by 8
    Tgt(u8),
    /// The following byte
    Imm8(u8),
    /// The following 2 bytes, LE order
    Imm16(u16),
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Instruction {
    AdcA(OpR8),
    AddA(OpR8),
    AndA(OpR8),
    Bit(Operand),
    CallCond(OpCond, Operand),
    Call(Operand),
    Ccf,
    CpA(OpR8),
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
    OrA(OpR8),
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
    SbcA(OpR8),
    Scf,
    Set(Operand, Operand),
    Sla(Operand),
    Sra(Operand),
    Srl(Operand),
    Stop,
    SubA(OpR8),
    Swap(Operand),
    XorA(OpR8),
}

impl TryFrom<u8> for OpR8 {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use OpR8::*;
        match value {
            0 => Ok(B),
            1 => Ok(C),
            2 => Ok(D),
            3 => Ok(E),
            4 => Ok(H),
            5 => Ok(L),
            6 => Ok(HL),
            7 => Ok(A),
            _ => Err(String::from("Values greater than 7 not accepted")),
        }
    }
}
