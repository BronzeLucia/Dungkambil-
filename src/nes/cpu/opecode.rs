


use std::collections::HashMap;

#[derive(Debug)]
pub struct Opecode {
    pub name: Instruction,
    pub mode: Addressing,
    pub cycle: u8,
}

#[derive(Debug)]
pub enum Instruction {
    LDA,
    LDX,
    LDY,
    STA,
    STX,
    STY,
    TXA,
    TYA,
    TXS,
    TAY,
    TAX,
    TSX,
    PHP,
    PLP,
    PHA,
    PLA,
    ADC,
    SBC,
    CPX,
    CPY,
    CMP,
    AND,
    EOR,
    ORA,
    BIT,
    ASL,
    LSR,
    ROL,
    ROR,
    INX,
    INY,
    INC,
    DEX,
    DEY,
    DEC,
    CLC,
   