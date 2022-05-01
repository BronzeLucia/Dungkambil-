


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
    CLI,
    CLV,
    SEC,
    SEI,
    NOP,
    BRK,
    JSR,
    JMP,
    RTI,
    RTS,
    BPL,
    BMI,
    BVC,
    BVS,
    BCC,
    BCS,
    BNE,
    BEQ,
    SED,
    CLD,
    LAX,
    SAX,
    DCP,
    ISB,
    SLO,
    RLA,
    SRE,
    RRA,
}

#[derive(Debug, PartialEq)]
pub enum Addressing {
    Immediate,
    ZeroPage,
    Relative,
    Implied,
    Absolute,
    Accumulator,
    ZeroPageX,
    ZeroPageY,
    AbsoluteX,
    AbsoluteY,
    PreIndexedIndirect,
    PostIndexedIndirect,
    IndirectAbsolute,
}

lazy_static! {
        
        pub static ref MAP: HashMap<u8, Opecode> = {

    