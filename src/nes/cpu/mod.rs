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
  