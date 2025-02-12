#[cfg(test)]
mod tests;

use Flag::*;
use Register16::*;
use Register8::*;

pub struct Machine {
    registers: RegisterState,
}

/// Stores the register state
///
/// Look at: https://gbdev.io/pandocs/CPU_Registers_and_Flags.html for more info
struct RegisterState {
    af: u16,
    bc: u16,
    de: u16,
    hl: u16,
    sp: u16,
    pc: u16,
}

enum Register8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

enum Register16 {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

enum Flag {
    Zero,
    Subtraction,
    HalfCarry,
    Carry,
}

trait SetHalves {
    fn set_low(&mut self, value: u8);
    fn set_high(&mut self, value: u8);
}

impl SetHalves for u16 {
    fn set_low(&mut self, value: u8) {
        *self = (*self & (0xFF00)) + (value as u16);
    }

    fn set_high(&mut self, value: u8) {
        *self = (*self & (0x00FF)) + ((value as u16) << 8);
    }
}

impl RegisterState {
    pub fn new() -> Self {
        RegisterState {
            af: 0,
            bc: 0,
            de: 0,
            hl: 0,
            sp: 0,
            pc: 0,
        }
    }
    pub fn get_u8(&self, register: Register8) -> u8 {
        (match register {
            A => self.af >> 8,
            B => self.bc >> 8,
            C => self.bc & 0xFF,
            D => self.de >> 8,
            E => self.de & 0xFF,
            H => self.hl >> 8,
            L => self.hl & 0xFF,
        }) as u8
    }

    pub fn set_u8(&mut self, register: Register8, value: u8) {
        match register {
            A => self.af.set_high(value),
            B => self.bc.set_high(value),
            C => self.bc.set_low(value),
            D => self.de.set_high(value),
            E => self.de.set_low(value),
            H => self.hl.set_high(value),
            L => self.hl.set_low(value),
        };
    }

    pub fn set_u16(&mut self, register: Register16, value: u16) {
        match register {
            AF => self.af = value,
            BC => self.bc = value,
            DE => self.de = value,
            HL => self.hl = value,
            SP => self.sp = value,
            PC => self.pc = value,
        };
    }

    pub fn get_u16(&self, register: Register16) -> u16 {
        match register {
            AF => self.af,
            BC => self.bc,
            DE => self.de,
            HL => self.hl,
            SP => self.sp,
            PC => self.pc,
        }
    }

    pub fn get_flag(&self, flag: Flag) -> bool {
        let bit = match flag {
            Zero => 7,
            Subtraction => 6,
            HalfCarry => 5,
            Carry => 4,
        } - 1;
        self.af & (1 << bit) != 0
    }
}
