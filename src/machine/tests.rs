use std::array;

use rand::Rng;

use super::{Machine, RegisterState};

#[test]
fn set_and_read_u8_registers() {
    use crate::machine::registers::Register8::*;

    const REG_COUNT: usize = 7;

    let mut rng = rand::rng();

    let reg_bits: [u8; REG_COUNT] = array::from_fn(|_| rng.random());
    let mut machine = Machine {
        registers: RegisterState::new(),
    };

    machine.registers.set_u8(A, reg_bits[0]);
    machine.registers.set_u8(B, reg_bits[1]);
    machine.registers.set_u8(C, reg_bits[2]);
    machine.registers.set_u8(D, reg_bits[3]);
    machine.registers.set_u8(E, reg_bits[4]);
    machine.registers.set_u8(H, reg_bits[5]);
    machine.registers.set_u8(L, reg_bits[6]);

    assert_eq!(machine.registers.get_u8(A), reg_bits[0]);
    assert_eq!(machine.registers.get_u8(B), reg_bits[1]);
    assert_eq!(machine.registers.get_u8(C), reg_bits[2]);
    assert_eq!(machine.registers.get_u8(D), reg_bits[3]);
    assert_eq!(machine.registers.get_u8(E), reg_bits[4]);
    assert_eq!(machine.registers.get_u8(H), reg_bits[5]);
    assert_eq!(machine.registers.get_u8(L), reg_bits[6]);
}

#[test]
fn set_and_read_u16_registers() {
    use crate::machine::registers::Register16::*;

    const REG_COUNT: usize = 6;
    let mut rng = rand::rng();
    let reg_bits: [u16; REG_COUNT] = array::from_fn(|_| rng.random::<u16>());

    let mut machine = Machine {
        registers: RegisterState::new(),
    };

    machine.registers.set_u16(AF, reg_bits[0]);
    machine.registers.set_u16(BC, reg_bits[1]);
    machine.registers.set_u16(DE, reg_bits[2]);
    machine.registers.set_u16(HL, reg_bits[3]);
    machine.registers.set_u16(SP, reg_bits[4]);
    machine.registers.set_u16(PC, reg_bits[5]);

    assert_eq!(machine.registers.get_u16(AF), reg_bits[0]);
    assert_eq!(machine.registers.get_u16(BC), reg_bits[1]);
    assert_eq!(machine.registers.get_u16(DE), reg_bits[2]);
    assert_eq!(machine.registers.get_u16(HL), reg_bits[3]);
    assert_eq!(machine.registers.get_u16(SP), reg_bits[4]);
    assert_eq!(machine.registers.get_u16(PC), reg_bits[5]);
}

#[test]
fn get_flags() {
    use crate::machine::registers::Flag::*;
    use crate::machine::registers::Register16;
    let mut machine = Machine {
        registers: RegisterState::new(),
    };
    machine.registers.set_u16(Register16::AF, 0b00101000);
    assert_eq!(machine.registers.get_flag(Zero), false);
    assert_eq!(machine.registers.get_flag(Subtraction), true);
    assert_eq!(machine.registers.get_flag(HalfCarry), false);
    assert_eq!(machine.registers.get_flag(Carry), true);
}
