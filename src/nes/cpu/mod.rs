mod fetch;
mod instructions;
mod opecode;

use self::fetch::*;
use self::instructions::*;
use self::opecode::*;
use std::fmt::Debug;

use super::bus::cpu_bus::CpuBus;
use super::cpu_registers::CpuRegisters;
use super::types::Data;

pub fn reset<T: CpuRegisters, U: CpuBus>(registers: &mut T, bus: &mut U) {
    let pc = bus.read_word(0xFFFC);
    registers.set_PC(pc);
}

pub fn run<T: CpuRegisters + Debug, U: CpuBus>(
    registers: &mut T,
    bus: &mut U,
    nmi: &mut bool,
) -> Data {
    if *nmi {
        process_nmi(registers, bus);
        *nmi = false;
    }
    let code = fetch(registers, bus);
    let ref map = opecode::MAP;
    let code = &*map.get(&code).unwrap();
    let operand = fetch_operand(&code, registers, bus);
    // println!("opecode = {}, {:?} pc = {:x}, operand = {:x}", &_code, code.name, &registers.get_PC(), operand);
    match code.name {
        Instruction::LDA if code.mode == Addressing::Immediate => lda_imm(operand, registers),
        Instruction::LDA => lda(operand, registers, bus),
        Instruction::LDX if code.mode == Addressing::Immediate => ldx_imm(operand, registers),
        Instruction::LDX => ldx(operand, registers, bus),
        Instruction::LDY if code.mode == Addressing::Immediate => ldy_imm(operand, registers),
        Instruction::LDY => ldy(operand, registers, bus),
        Instruction::STA => sta(operand, registers, bus),
        Instruction::STX => stx(operand, registers, bus),
        Instruction::STY => sty(operand, registers, bus),
        Instruction::TXA => txa(registers),
        Instruction::TYA => tya(registers),
        Instruction::TXS => txs(registers),
        Instruction::TAY => tay(registers),
        Instruction::TAX => tax(registers),
        Instruction::TSX => tsx(registers),
        Instruction::PHP => php(registers, bus),
        Instruction::PLP => plp(registers, bus),
        Instruction::PHA => pha(registers, bus),
        Instruction::PLA => pla(registers, bus),
        Instruction::ADC if code.mode == Addressing::Immediate => adc_imm(operand, registers),
        Instruction::ADC => adc(operand, registers, bus),
        Instruction::SBC if code.mode == Addressing::Immediate => sbc_imm(operand, registers),
        Instruction::SBC => sbc(operand, registers, bus),
        Instruction::CPX if code.mode == Addressing::Immediate => cpx_imm(operand, registers),
        Instruction::CPX => cpx(operand, registers, bus),
        Instruction::CPY if code.mode == Addressing::Immediate => cpy_imm(operand, registers),
        Instruction::CPY => cpy(operand, registers, bus),
        Instruction::CMP if code.mode == Addressing::Immediate => cmp_imm(operand, registers),
        Instruction::CMP => cmp(operand, registers, bus),
        Instruction::AND if code.mode == Addressing::Immediate => and_imm(operand, registers),
        Instruction::AND => and(operand, registers, bus),
        Instruction::EOR if code.mode == Addressing::Immediate => eor_imm(operand, registers),
        Instruction::EOR => eor(operand, registers, bus),
        Instruction::ORA if code.mode == Addressing::Immediate => ora_imm(operand, registers),
        Instruction::ORA => ora(operand, registers, bus),
        Instruction::BIT => bit(operand, registers, bus),
        Instruction::ASL if code.mode == Addressing::Accumulator => asl_acc(registers),
        Instruction::ASL => asl(operand, registers, bus),
        Instruction::LSR if code.mode == Addressing::Accumulator => lsr_acc(registers),
        Instruction::LSR => lsr(operand, registers, bus),
        In