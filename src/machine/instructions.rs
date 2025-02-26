use super::DecodeError;

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
    /// adc a, r8
    AdcA(OpR8),
    /// add a, r8
    AddA(OpR8),
    /// add hl, r16
    AddHlR16(OpR16),
    /// and a, r8
    AndA(OpR8),
    Bit(Operand),
    CallCond(OpCond, Operand),
    Call(Operand),
    Ccf,
    CpA(OpR8),
    Cpl,
    Daa,
    /// dec r16
    DecR16(OpR16),
    Di,
    Ei,
    Halt,
    /// inc r16
    IncR16(OpR16),
    Jp(Operand),
    JpCond(OpCond, Operand),
    Jr(Operand),
    JrCond(OpCond, Operand),
    /// ld r16, imm16
    LdR16Imm16(OpR16),
    /// ld [r16mem], a
    LdR16MemA(OpR16Mem),
    /// ld a, [r16mem]
    LdAR16Mem(OpR16Mem),
    /// ld [imm16], sp
    LdImm16SP,
    Ldh(Operand, Operand),
    Nop,
    /// or a, r8
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
    /// sbc a, r8
    SbcA(OpR8),
    Scf,
    Set(Operand, Operand),
    Sla(Operand),
    Sra(Operand),
    Srl(Operand),
    Stop,
    /// sub a, r8
    SubA(OpR8),
    Swap(Operand),
    /// xor a, r8
    XorA(OpR8),
}

/// This trait would be redundant but we want to keep our `DecodeError` type private
/// because there is no valid reason to expose it to other parts of our code
pub trait DecodeFromU8
where
    Self: Sized,
{
    fn decode_from(value: u8) -> Result<Self, DecodeError>;
}

impl DecodeFromU8 for OpR8 {
    fn decode_from(value: u8) -> Result<Self, DecodeError> {
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
            _ => Err(DecodeError::InvalidR8),
        }
    }
}

impl DecodeFromU8 for OpR16 {
    fn decode_from(value: u8) -> Result<Self, DecodeError> {
        use OpR16::*;
        match value {
            0 => Ok(BC),
            1 => Ok(DE),
            2 => Ok(HL),
            3 => Ok(SP),
            _ => Err(DecodeError::InvalidR16),
        }
    }
}

impl DecodeFromU8 for OpR16Mem {
    fn decode_from(value: u8) -> Result<Self, DecodeError> {
        use OpR16Mem::*;
        match value {
            0 => Ok(BC),
            1 => Ok(DE),
            2 => Ok(HLPlus),
            3 => Ok(HLMinus),
            _ => Err(DecodeError::InvalidR16),
        }
    }
}

impl DecodeFromU8 for OpR16Stk {
    fn decode_from(value: u8) -> Result<Self, DecodeError> {
        use OpR16Stk::*;
        match value {
            0 => Ok(BC),
            1 => Ok(DE),
            2 => Ok(HL),
            3 => Ok(AF),
            _ => Err(DecodeError::InvalidR16),
        }
    }
}
