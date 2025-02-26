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
pub enum Instruction {
    /// adc a, r8
    AdcAR8(OpR8),
    /// adc a, imm8
    AdcAImm8,
    /// add a, r8
    AddAR8(OpR8),
    /// add hl, r16
    AddHlR16(OpR16),
    /// add a, imm8
    AddAImm8,
    /// and a, r8
    AndAR8(OpR8),
    /// and a, imm8
    AndAImm8,
    /// bit b3, r8
    BitB3R8(u8, OpR8),
    /// call cond, imm16
    CallCondImm16(OpCond, u16),
    /// call imm16
    CallImm16(u16),
    /// ccf
    Ccf,
    /// cp a, r8
    CpAR8(OpR8),
    /// cp a, imm8
    CpAImm8,
    /// cpl
    Cpl,
    /// daa
    Daa,
    /// dec r16
    DecR16(OpR16),
    /// dec r8
    DecR8(OpR8),
    /// di
    Di,
    /// ei
    Ei,
    /// halt
    Halt,
    /// inc r16
    IncR16(OpR16),
    /// inc r8
    IncR8(OpR8),
    /// jr imm8
    JrImm8,
    /// jr cond, imm8
    JrCondImm8(OpCond),
    /// jp cond, imm16
    JpCondImm16(OpCond, u16),
    /// jp imm16
    JpImm16(u16),
    /// jp hl
    JpHl,
    /// ld r16, imm16
    LdR16Imm16(OpR16),
    /// ld [r16mem], a
    LdR16MemA(OpR16Mem),
    /// ld a, [r16mem]
    LdAR16Mem(OpR16Mem),
    /// ld [imm16], sp
    LdImm16SP,
    /// ldh a, [imm8]
    LdhAImm8(u8),
    /// ldh [imm8], a
    LdhImm8A(u8),
    /// ldh [c], a
    LdhCA,
    /// ldh a, [c]
    LdhAC,
    /// nop
    Nop,
    /// or a, r8
    OrAR8(OpR8),
    /// or a, imm8
    OrAImm8,
    /// pop r16stk
    Pop(OpR16Stk),
    /// push r16stk
    Push(OpR16Stk),
    /// res b3, r8
    ResB3R8(u8, OpR8),
    /// ret cond
    RetCond(OpCond),
    /// ret
    Ret,
    /// reti
    Reti,
    /// rl r8
    RlR8(OpR8),
    /// rla
    Rla,
    /// rlc r8
    RlcR8(OpR8),
    /// rlca
    Rlca,
    /// rr r8
    RrR8(OpR8),
    /// rrca
    Rrca,
    /// rst tgt3
    RstTgt3(u8), // Target address divided by 8 (0x00-0x38)
    /// sbc a, r8
    SbcAR8(OpR8),
    /// sbc a, imm8
    SbcAImm8,
    /// scf
    Scf,
    /// set b3, r8
    SetB3R8(u8, OpR8),
    /// sla r8
    SlaR8(OpR8),
    /// sra r8
    SraR8(OpR8),
    /// srl r8
    SrlR8(OpR8),
    /// stop
    Stop,
    /// sub a, r8
    SubAR8(OpR8),
    /// sub a, imm8
    SubAImm8,
    /// swap r8
    SwapR8(OpR8),
    /// xor a, r8
    XorAR8(OpR8),
    /// xor a, imm8
    XorAImm8,
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
