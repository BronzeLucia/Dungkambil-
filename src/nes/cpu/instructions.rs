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

pub fn lda<T: CpuRegisters, U: CpuBus>(operand