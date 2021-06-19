use super::super::cpu_registers::CpuRegisters;
use super::super::bus::cpu_bus::CpuBus;
use super::super::types::{Data, Addr, Word};
use super::super::helper::*;

pub fn process_nmi<T: CpuRegisters, U: CpuBus>(registers: &mut T, bus: &mut U) {
    registers.set_break(false);
    push((registers.get_PC() >> 8) as u8, registers, bus);
    push(registers.get_PC() as u8, registers, bus);
    push_status(registers, bus);
    registers.set_interrupt(true);
    let next = bus.read_word(0xFFFA);
    registers.set_PC(next);
}

pub fn lda<T: CpuRegisters, U: CpuBus>(operand: Word, registers: &mut T, bus: &mut U) {
    let computed = bus.read(operand);
    registers
        .set_A(computed)
        .update_negative_by(computed)
        .update_zero_by(computed);
}

pub fn lda_imm<T: CpuRegisters>(operand: Word, registers: &mut T) {
    registers
        .set_A(operand as Data)
        .update_negative_by(operand as Data)
        .update_zero_by(operand as Data);
}

pub fn ldx<T: CpuRegisters, U: CpuBus>(operand: Word, registers: &mut T, bus: &mut U) {
    let computed = bus.read(operand);
    registers
        .set_X(computed)
        .update_negative_by(computed)
        .update_zero_by(computed);
}

pub fn ldx_imm<T: CpuRegisters>(operand: Word, registers: &mut T) {
    registers
        .set_X(operand as Data)
        .update_negative_by(operand as Data)
        .update_zero_by(operand as Data);
}

pub fn ldy<T: CpuRegisters, U: CpuBus>(operand: Word, registers: &mut T, bus: &mut U) {
    let computed = bus.read(operand);
    registers
        .set_Y(computed)
        .update_negative_by(computed)
        .update_zero_by(computed);
}

pub fn ldy_imm<T: CpuRegisters>(operand: Word, registers: &mut T) {
    registers
        .set_Y(operand as Data)
        .update_negative_by(operand as Data)
        .update_zero_by(operand as Data);
}

pub fn sta<T: CpuRegisters, U: CpuBus>(operand: Word, registers: &mut T, bus: &mut U) {
    bus.write(operand, registers.get_A());
}

pub fn stx<T: CpuRegisters, U: CpuBus>(operand: Word, registers: &mut T, bus: &mut U) {
    bus.write(operand, registers.get_X());
}

pub fn sty<T: CpuRegisters, U: CpuBus>(operand: Word, registers: &mut T, bus: &mut U) {
    bus.write(operand, registers.get_Y());
}

pub fn txa<T: CpuRegisters>(registers: &mut T) {
    let x = registers.get_X();
    registers
        .set_A(x)
        .update_negative_by(x)
        .update_zero_by(x);
}

pub fn tya<T: CpuRegisters>(registers: &mut T) {
    let y = registers.get_Y();
    registers
        .set_A(y)
        .update_negative_by(y)
        .update_zero_by(y);
}

pub fn txs<T: CpuRegisters>(registers: &mut T) {
    let x = registers.get_X();
    registers.set_SP(x);
}

pub fn tay<T: CpuRegisters>(registers: &mut T) {
    let acc = registers.get_A();
    registers
        .set_Y(acc)
        .update_negative_by(acc)
        .update_zero_by(acc);
}

pub fn tax<T: CpuRegisters>(registers: &mut T) {
    let acc = registers.get_A();
    registers
        .set_X(acc)
        .update_negative_by(acc)
        .update_zero_by(acc);
}

pub fn tsx<T: CpuRegisters>(registers: &mut T) {
    let sp = registers.get_SP();
    registers
        .set_X(sp)
        .update_negative_by(sp)
        .update_zero_by(sp);
}

pub fn php<T: CpuRegisters, U: CpuBus>(registers: &mut T, bus: &mut U) {
    registers.set_break(true);
    push_status(registers, bus);
}

pub fn plp<T: CpuRegisters, U: CpuBus>(registers: &mut T, bus: &mut U) {
    registers.set_reserved(true);
    let status = pop(registers, bus);
    registers.set_P(status);
}

pub fn pha<T: CpuRegisters, U: CpuBus>(registers: &mut T, bus: &mut U) {
    let acc = registers.get_A();
    push(acc, registers, bus);
}

pub fn pla<T: CpuRegisters, U: CpuBus>(registers: &mut T, bus: &mut U) {
    let v = pop(registers, bus);
    registers
        .set_A(v)
        .update_negative_by(v)
        .update_zero_by(v);
}

pub fn adc_imm<T: CpuRegisters>(operand: Word, registers: &mut T) {
    let computed = (operand as u16) + registers.get_A() as u16 +
                   bool_to_u8(registers.get_carry()) as u16;
    let acc = registers.get_A();
    registers
        .set_overflow(!(((acc ^ (operand as Data)) & 0x80) != 0) &&
                      (((acc ^ computed as Data) & 0x80)) != 0)
        .update_negative_by(computed as Data)
        .update_zero_by(computed as Data)
        .set_carry(computed > 0xFF)
        .set_A(computed as Data);
}

pub fn adc<T: CpuRegisters, U: CpuBus>(operand: Word, registers: &mut T, bus: &mut U) {
    let fetched = bus.read(operand);
    let computed = fetched as u16 + registers.get_A() as u16 +
                   bool_to_u8(registers.get_carry()) as u16;
    let acc = registers.get_A();
    registers
        .set_overflow(!(((acc ^ (fetched as Data)) & 0x80) != 0) &&
                      (((acc ^ computed as Data) & 0x80)) != 0)
        .update_negative_by(computed as Data)
        .update_zero_by(computed as Data)
        .set_carry(computed > 0xFF)
        .set_A(computed as Data);
}

pub fn sbc_imm<T: CpuRegisters>(operand: Word, registers: &mut T) {
    let computed = registers.get_A() as i16 - (operand as i16) -
                   bool_to_u8(!registers.get_carry()) as i16;
    let acc = registers.get_A();
    registers
        .set_overflow((((acc ^ (operand as Data)) & 0x80) != 0) &&
                      (((acc ^ computed as Data) & 0x80)) != 0)
        .update_negative_by(computed as Data)
        .update_zero_by(computed as Data)
        .set_carry(computed >= 0 as i16)
        .set_A(computed as Data);
}

pub fn sbc<T: CpuRegisters, U: CpuBus>(operand: Word, registers: &mut T, bus: &mut U) {
    let fetched = bus.read(operand);
    let computed = registers.get_A() as i16 - fetched as i16 -
                   bool_to_u8(!registers.get_carry()) as i16;
    let acc = registers.get_A();
    registers
        .set_overflow((((acc ^ (fetched as Data)) & 0x80) != 0) &&
                      (((acc ^ computed as Data) & 0x80)) != 0)
        .update_negative_by(computed as Data)
        .update_zero_by(computed as Data)
        .set_carry(computed >= 0 as i16)
        .set_A(computed as Data);
}

pub fn cpx_imm<T: CpuRegisters>(operand: Word, registers: &mut T) {
    let computed = registers.get_X() as i16 - (operand as i16);
    registers
        .update_negative_by(computed as Data)
        .update_zero_by(computed as Data)
        .set_carry(computed >= 0 as i16);
}

pub fn cpx<T: CpuRegisters, U: CpuBus>(operand: Word, registers: &mut T, bus: &mut U) {
    let fetched = bus.read(operand);
    let computed = registers.get_X() as i16 - fetched as i16;
    registers
        .update_negative_by(computed as Data)
        .update_zero_by(computed as Data)
        .set_carry(computed >= 0 as i16);
}

pub fn cpy_imm<T: CpuRegisters>(operand: Word, registers: &mut T) {
    let computed = registers.get_Y() as i16 - (operand as i16);
    registers
        .update_negative_by(computed as Data)
        .update_zero_by(computed as Data)
        .set_carry(computed >= 0 as i16);
}

pub fn cpy<T: CpuRegisters, U: CpuBus>(operand: Word, registers: &mut T, bus: &mut U) {
    let fetched = bus.read(operand);
    let computed = registers.get_Y() as i16 - fetched as i16;
    registers
        .update_negative_by(computed as Data)
        .update_zero_by(computed as Data)
        .set_carry(computed >= 0 as i16);
}

pub fn cmp_imm<T: CpuRegisters>(operand: Word, registers: &mut T) {
    let computed = (registers.get_A() as i16) - (operand as i16);
    registers
        .update_negative_by(computed as Data)
        .update_zero_by(computed as Data)
        .set_carry(computed >= 0 as i16);
}

pub fn cmp<T: CpuRegisters, U: CpuBus>(operand: Word, registers: &mut T, bus: &mut U) {
    let fetched = bus.read(operand);
    let computed = (registers.get_A() as i16) - (fetched as i16);
    registers
        .update_negative_by(computed as Data)
        .update_zero_by(computed as Data)
        .set_carry(computed >= 0 as i16);
}

pub fn and_imm<T: CpuRegisters>(