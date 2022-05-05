


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

            #[cfg_attr(rustfmt, rustfmt_skip)]
            let cycles: Vec<u8> =
                vec![7, 6, 2, 8, 3, 3, 5, 5, 3, 2, 2, 2, 4, 4, 6, 6, 2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7,
                     4, 4, 6, 7, 6, 6, 2, 8, 3, 3, 5, 5, 4, 2, 2, 2, 4, 4, 6, 6, 2, 5, 2, 8, 4, 4, 6, 6,
                     2, 4, 2, 7, 4, 4, 6, 7, 6, 6, 2, 8, 3, 3, 5, 5, 3, 2, 2, 2, 3, 4, 6, 6, 2, 5, 2, 8,
                     4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 6, 7, 6, 6, 2, 8, 3, 3, 5, 5, 4, 2, 2, 2, 5, 4, 6, 6,
                     2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 6, 7, 2, 6, 2, 6, 3, 3, 3, 3, 2, 2, 2, 2,
                     4, 4, 4, 4, 2, 6, 2, 6, 4, 4, 4, 4, 2, 4, 2, 5, 5, 4, 5, 5, 2, 6, 2, 6, 3, 3, 3, 3,
                     2, 2, 2, 2, 4, 4, 4, 4, 2, 5, 2, 5, 4, 4, 4, 4, 2, 4, 2, 4, 4, 4, 4, 4, 2, 6, 2, 8,
                     3, 3, 5, 5, 2, 2, 2, 2, 4, 4, 6, 6, 2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7,
                     2, 6, 3, 8, 3, 3, 5, 5, 2, 2, 2, 2, 4, 4, 6, 6, 2, 5, 2, 8, 4, 4, 6, 6, 2