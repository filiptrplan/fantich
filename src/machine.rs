mod instructions;
mod registers;
#[cfg(test)]
mod tests;

use std::usize;

use instructions::Instruction;
use registers::RegisterState;

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
