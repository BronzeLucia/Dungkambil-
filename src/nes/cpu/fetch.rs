
use super::opecode::*;
use super::super::cpu_registers::CpuRegisters;
use super::super::bus::cpu_bus::CpuBus;
use super::super::types::{Data, Addr, Word};

pub fn fetch<T: CpuRegisters, U: CpuBus>(registers: &mut T, bus: &mut U) -> Data {
    let code = bus.read(registers.get_PC());
    registers.inc_PC();
    code
}

pub fn fetch_operand<T: CpuRegisters, U: CpuBus>(code: &Opecode,
                                                 registers: &mut T,
                                                 bus: &mut U)
                                                 -> Word {
    match code.mode {
        Addressing::Accumulator => 0x0000,
        Addressing::Implied => 0x0000,
        Addressing::Immediate => fetch(registers, bus) as Word,