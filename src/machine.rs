mod instructions;
mod registers;
#[cfg(test)]
mod tests;

use std::{error::Error, usize};

use instructions::{DecodeFromU8, Instruction, OpR16, OpR16Mem, OpR8};
use registers::{Register16, RegisterState};
use thiserror::Error;

pub struct Machine {
    registers: RegisterState,
    ram: Vec<u8>,
    cycle_countdown: u8,
}

#[derive(Debug, Error, PartialEq, Eq)]
enum DecodeError {
    #[error("invalid R8 operand. only values from 0 to 7 are allowed")]
    InvalidR8,
    #[error("invalid R16 operand. only values from 0 to 3 are allowed")]
    InvalidR16,
    #[error("found no matching instruction for: {0:#08b}")]
    NoMatchingInstruction(u8),
}

trait MatchBits {
    /// Returns true if the bits where mask is 1 exactly match the bits in `bits`
    /// ```
    /// assert!((0b00110011).match_bits(0b00110000, 0b11110000))
    /// ```
    fn match_bits(&self, bits: Self, mask: Self) -> bool;
}

/// Trait for only just extracting certain bits from a type
trait ExtractBits {
    /// Extracts the bits from bits_start to bits_end inclusive and shifts them
    /// to be at the bottom
    /// ```
    /// assert_eq!((0b00110000).extract_bits(1,5), 0b00011000)
    /// ```
    fn extract_bits(&self, bits_start: u8, bits_end: u8) -> Self;
}

impl MatchBits for u8 {
    fn match_bits(&self, bits: Self, mask: Self) -> bool {
        ((self & mask) ^ (bits & mask)) == 0
    }
}

impl ExtractBits for u8 {
    fn extract_bits(&self, bits_start: u8, bits_end: u8) -> Self {
        (self << (7 - bits_end)) >> (7 - bits_end + bits_start)
    }
}

impl Machine {
    pub fn new() -> Self {
        Machine {
            registers: RegisterState::new(),
            ram: vec![0; 0xFFFF + 1],
            cycle_countdown: 0,
        }
    }

    pub fn load_ram(&mut self, initial_ram: &[u8]) -> Result<(), String> {
        if initial_ram.len() > self.ram.len() {
            return Err(String::from("The initial RAM is too big."));
        }
        self.ram[0..initial_ram.len()].copy_from_slice(initial_ram);
        Ok(())
    }

    fn read_ram(&self, addr: u16) -> u8 {
        self.ram[addr as usize]
    }

    fn decode_instruction(&self) -> Result<Instruction, DecodeError> {
        // TODO: This way of checking for instructions requires a lot of branching
        // maybe think of a way that would be more elegant?
        let byte_inst = self.read_ram(self.registers.get_u16(Register16::PC));
        // Block 0
        if byte_inst.match_bits(0b00000000, 0b11000000) {
            if byte_inst == 0 {
                return Ok(Instruction::Nop);
            }
            // Load instructions
            if byte_inst.match_bits(0b00000001, 0b00001111) {
                return Ok(Instruction::LdR16Imm16(OpR16::decode_from(
                    byte_inst.extract_bits(4, 5),
                )?));
            } else if byte_inst.match_bits(0b00000010, 0b00001111) {
                return Ok(Instruction::LdR16MemA(OpR16Mem::decode_from(
                    byte_inst.extract_bits(4, 5),
                )?));
            } else if byte_inst.match_bits(0b00001010, 0b00001111) {
                return Ok(Instruction::LdAR16Mem(OpR16Mem::decode_from(
                    byte_inst.extract_bits(4, 5),
                )?));
            } else if byte_inst == 0b00001000 {
                return Ok(Instruction::LdImm16SP);
            // Inc, Dec, Add r16
            } else if byte_inst.match_bits(0b00000011, 0b00001111) {
                return Ok(Instruction::IncR16(OpR16::decode_from(
                    byte_inst.extract_bits(4, 5),
                )?));
            } else if byte_inst.match_bits(0b00001011, 0b00001111) {
                return Ok(Instruction::DecR16(OpR16::decode_from(
                    byte_inst.extract_bits(4, 5),
                )?));
            } else if byte_inst.match_bits(0b00001001, 0b00001111) {
                return Ok(Instruction::AddHlR16(OpR16::decode_from(
                    byte_inst.extract_bits(4, 5),
                )?));
            }
        }
        // Block 2
        if byte_inst.match_bits(0b10000000, 0b11000000) {
            let operand = OpR8::decode_from(byte_inst.extract_bits(0, 2))?;

            if byte_inst.match_bits(0b00000000, 0b00111000) {
                return Ok(Instruction::AddA(operand));
            } else if byte_inst.match_bits(0b00001000, 0b00111000) {
                return Ok(Instruction::AdcA(operand));
            } else if byte_inst.match_bits(0b00010000, 0b00111000) {
                return Ok(Instruction::SubA(operand));
            } else if byte_inst.match_bits(0b00011000, 0b00111000) {
                return Ok(Instruction::SbcA(operand));
            } else if byte_inst.match_bits(0b00100000, 0b00111000) {
                return Ok(Instruction::AndA(operand));
            } else if byte_inst.match_bits(0b00101000, 0b00111000) {
                return Ok(Instruction::XorA(operand));
            } else if byte_inst.match_bits(0b00110000, 0b00111000) {
                return Ok(Instruction::OrA(operand));
            } else if byte_inst.match_bits(0b00111000, 0b00111000) {
                return Ok(Instruction::CpA(operand));
            }
        }
        Err(DecodeError::NoMatchingInstruction(byte_inst))
    }

    fn cycle(&mut self) {
        todo!()
    }
}
