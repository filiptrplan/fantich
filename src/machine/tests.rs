use std::array;

use crate::machine::registers::Register16;

use super::Machine;
use rand::Rng;

/*
* MISC TESTS
*/

#[test]
fn match_bits() {
    use crate::machine::MatchBits;
    assert!((0b00110011 as u8).match_bits(0b00110000, 0b11110000));
    assert!(!(0b00110011 as u8).match_bits(0b00110000, 0b11110010));
}

#[test]
fn extract_bits() {
    use crate::machine::ExtractBits;
    assert_eq!((0b00110000).extract_bits(1, 5), 0b00011000);
}

/*
* REGISTER TESTS
*/

#[test]
fn set_and_read_u8_registers() {
    use crate::machine::registers::Register8::*;

    const REG_COUNT: usize = 7;

    let mut rng = rand::rng();

    let reg_bits: [u8; REG_COUNT] = array::from_fn(|_| rng.random());
    let mut machine = Machine::new();

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

    let mut machine = Machine::new();

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
    let mut machine = Machine::new();
    machine.registers.set_u16(Register16::AF, 0b00101000);
    assert_eq!(machine.registers.get_flag(Zero), false);
    assert_eq!(machine.registers.get_flag(Subtraction), true);
    assert_eq!(machine.registers.get_flag(HalfCarry), false);
    assert_eq!(machine.registers.get_flag(Carry), true);
}

/*
* INSTRUCTION TESTS
*/

fn setup_machine(initial_rom: &[u8]) -> Machine {
    let mut machine = Machine::new();
    let res = machine.load_ram(&initial_rom);
    if let Err(err) = res {
        panic!("{}", err);
    }
    machine
}

#[test]
fn decode_block2() {
    use crate::machine::instructions::*;
    let mut machine = setup_machine(&[
        0b10000011, // add a, e
        0b10001011, // adc a, e
        0b10010011, // sub a, e
        0b10011011, // sbc a, e
        0b10100011, // and a, e
        0b10101011, // xor a, e
        0b10110011, // or a, e
        0b10111011, // cp a, e
    ]);

    let expected_instructions = [
        Instruction::AddAR8(OpR8::E),
        Instruction::AdcAR8(OpR8::E),
        Instruction::SubAR8(OpR8::E),
        Instruction::SbcAR8(OpR8::E),
        Instruction::AndAR8(OpR8::E),
        Instruction::XorAR8(OpR8::E),
        Instruction::OrAR8(OpR8::E),
        Instruction::CpAR8(OpR8::E),
    ];

    for expected in expected_instructions {
        assert_eq!(machine.decode_instruction(), Ok(expected));
        // Manually increase program counter because we are only decoding
        machine.registers.inc_u16(Register16::PC);
    }
}

#[test]
fn decode_block0() {
    use crate::machine::instructions::*;
    let mut machine = setup_machine(&[
        0,          // nop
        0b00110001, // ld sp, imm16
        0b00110010, // ld hl-, a
        0b00111010, // ld a, hl-
        0b00001000, // ld [imm16], sp
        0b00000011, // inc bc
        0b00010011, // inc de
        0b00100011, // inc hl
        0b00110011, // inc sp
        0b00001011, // dec bc
        0b00011011, // dec de
        0b00101011, // dec hl
        0b00111011, // dec sp
        0b00001001, // add hl, bc
        0b00011001, // add hl, de
        0b00101001, // add hl, hl
        0b00111001, // add hl, sp
    ]);

    let expected_instructions = [
        Instruction::Nop,
        Instruction::LdR16Imm16(OpR16::SP),
        Instruction::LdR16MemA(OpR16Mem::HLMinus),
        Instruction::LdAR16Mem(OpR16Mem::HLMinus),
        Instruction::LdImm16SP,
        Instruction::IncR16(OpR16::BC),
        Instruction::IncR16(OpR16::DE),
        Instruction::IncR16(OpR16::HL),
        Instruction::IncR16(OpR16::SP),
        Instruction::DecR16(OpR16::BC),
        Instruction::DecR16(OpR16::DE),
        Instruction::DecR16(OpR16::HL),
        Instruction::DecR16(OpR16::SP),
        Instruction::AddHlR16(OpR16::BC),
        Instruction::AddHlR16(OpR16::DE),
        Instruction::AddHlR16(OpR16::HL),
        Instruction::AddHlR16(OpR16::SP),
    ];

    for expected in expected_instructions {
        assert_eq!(machine.decode_instruction(), Ok(expected));
        // Manually increase program counter because we are only decoding
        machine.registers.inc_u16(Register16::PC);
    }
}
