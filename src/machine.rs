mod registers;
#[cfg(test)]
mod tests;

use std::usize;

use registers::RegisterState;

enum OpR8 {
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
    A,
}

enum OpR16 {
    BC,
    DE,
    HL,
    SP,
}

enum OpR16Stk {
    BC,
    DE,
    HL,
    AF,
}

enum OpR16Mem {
    BC,
    DE,
    HLPlus,
    HLMinus,
}

enum OpCond {
    NZ,
    Z,
    NC,
    C,
}

enum Operand {
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

enum Instruction {
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

pub struct Machine {
    registers: RegisterState,
    ram: Vec<u8>,
    cycle_countdown: u8,
}

impl Machine {
    pub fn new() -> Self {
        Machine {
            registers: RegisterState::new(),
            ram: vec![0; 0xFFFF + 1],
            cycle_countdown: 0,
        }
    }

    fn read_ram(&self, addr: u16) -> u8 {
        self.ram[addr as usize]
    }

    fn decode_instruction(&self) -> Instruction {
        todo!()
    }
    fn cycle(&mut self) {
        todo!()
    }
}
