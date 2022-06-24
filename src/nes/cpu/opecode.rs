


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
                     2, 6, 3, 8, 3, 3, 5, 5, 2, 2, 2, 2, 4, 4, 6, 6, 2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7,
                     4, 4, 7, 7];            
            let mut m = HashMap::new();
            m.insert(0xA9, Opecode { name: Instruction::LDA, mode: Addressing::Immediate, cycle: cycles[0xA9] });
            m.insert(0xA5, Opecode { name: Instruction::LDA, mode: Addressing::ZeroPage, cycle: cycles[0xA5] });
            m.insert(0xB5, Opecode { name: Instruction::LDA, mode: Addressing::ZeroPageX, cycle: cycles[0xB5] });
            m.insert(0xAD, Opecode { name: Instruction::LDA, mode: Addressing::Absolute, cycle: cycles[0xAD] });
            m.insert(0xBD, Opecode { name: Instruction::LDA, mode: Addressing::AbsoluteX, cycle: cycles[0xBD] });
            m.insert(0xB9, Opecode { name: Instruction::LDA, mode: Addressing::AbsoluteY, cycle: cycles[0xB9] });
            m.insert(0xA1, Opecode { name: Instruction::LDA, mode: Addressing::PreIndexedIndirect, cycle: cycles[0xA1] });
            m.insert(0xB1, Opecode { name: Instruction::LDA, mode: Addressing::PostIndexedIndirect, cycle: cycles[0xB1] });
            m.insert(0xA2, Opecode { name: Instruction::LDX, mode: Addressing::Immediate, cycle: cycles[0xA2] });
            m.insert(0xA6, Opecode { name: Instruction::LDX, mode: Addressing::ZeroPage, cycle: cycles[0xA6] });
            m.insert(0xAE, Opecode { name: Instruction::LDX, mode: Addressing::Absolute, cycle: cycles[0xAE] });
            m.insert(0xB6, Opecode { name: Instruction::LDX, mode: Addressing::ZeroPageY, cycle: cycles[0xB6] });
            m.insert(0xBE, Opecode { name: Instruction::LDX, mode: Addressing::AbsoluteY, cycle: cycles[0xBE] });
            m.insert(0xA0, Opecode { name: Instruction::LDY, mode: Addressing::Immediate, cycle: cycles[0xA0] });
            m.insert(0xA4, Opecode { name: Instruction::LDY, mode: Addressing::ZeroPage, cycle: cycles[0xA4] });
            m.insert(0xAC, Opecode { name: Instruction::LDY, mode: Addressing::Absolute, cycle: cycles[0xAC] });
            m.insert(0xB4, Opecode { name: Instruction::LDY, mode: Addressing::ZeroPageX, cycle: cycles[0xB4] });
            m.insert(0xBC, Opecode { name: Instruction::LDY, mode: Addressing::AbsoluteX, cycle: cycles[0xBC] });
            m.insert(0x85, Opecode { name: Instruction::STA, mode: Addressing::ZeroPage, cycle: cycles[0x85] });
            m.insert(0x8D, Opecode { name: Instruction::STA, mode: Addressing::Absolute, cycle: cycles[0x8D] });
            m.insert(0x95, Opecode { name: Instruction::STA, mode: Addressing::ZeroPageX, cycle: cycles[0x95] });
            m.insert(0x9D, Opecode { name: Instruction::STA, mode: Addressing::AbsoluteX, cycle: cycles[0x9D] });
            m.insert(0x99, Opecode { name: Instruction::STA, mode: Addressing::AbsoluteY, cycle: cycles[0x99] });
            m.insert(0x81, Opecode { name: Instruction::STA, mode: Addressing::PreIndexedIndirect, cycle: cycles[0x81] });
            m.insert(0x91, Opecode { name: Instruction::STA, mode: Addressing::PostIndexedIndirect, cycle: cycles[0x91] });
            m.insert(0x86, Opecode { name: Instruction::STX, mode: Addressing::ZeroPage, cycle: cycles[0x86] });
            m.insert(0x8E, Opecode { name: Instruction::STX, mode: Addressing::Absolute, cycle: cycles[0x8E] });
            m.insert(0x96, Opecode { name: Instruction::STX, mode: Addressing::ZeroPageY, cycle: cycles[0x96] });
            m.insert(0x84, Opecode { name: Instruction::STY, mode: Addressing::ZeroPage, cycle: cycles[0x84] });
            m.insert(0x8C, Opecode { name: Instruction::STY, mode: Addressing::Absolute, cycle: cycles[0x8C] });
            m.insert(0x94, Opecode { name: Instruction::STY, mode: Addressing::ZeroPageX, cycle: cycles[0x94] });
            m.insert(0x8A, Opecode { name: Instruction::TXA, mode: Addressing::Implied, cycle: cycles[0x8A] });
            m.insert(0x98, Opecode { name: Instruction::TYA, mode: Addressing::Implied, cycle: cycles[0x98] });
            m.insert(0x9A, Opecode { name: Instruction::TXS, mode: Addressing::Implied, cycle: cycles[0x9A] });
            m.insert(0xA8, Opecode { name: Instruction::TAY, mode: Addressing::Implied, cycle: cycles[0xA8] });
            m.insert(0xAA, Opecode { name: Instruction::TAX, mode: Addressing::Implied, cycle: cycles[0xAA] });
            m.insert(0xBA, Opecode { name: Instruction::TSX, mode: Addressing::Implied, cycle: cycles[0xBA] });
            m.insert(0x08, Opecode { name: Instruction::PHP, mode: Addressing::Implied, cycle: cycles[0x08] });
            m.insert(0x28, Opecode { name: Instruction::PLP, mode: Addressing::Implied, cycle: cycles[0x28] });
            m.insert(0x48, Opecode { name: Instruction::PHA, mode: Addressing::Implied, cycle: cycles[0x48] });
            m.insert(0x68, Opecode { name: Instruction::PLA, mode: Addressing::Implied, cycle: cycles[0x68] });
            m.insert(0x69, Opecode { name: Instruction::ADC, mode: Addressing::Immediate, cycle: cycles[0x69] });
            m.insert(0x65, Opecode { name: Instruction::ADC, mode: Addressing::ZeroPage, cycle: cycles[0x65] });
            m.insert(0x6D, Opecode { name: Instruction::ADC, mode: Addressing::Absolute, cycle: cycles[0x6D] });
            m.insert(0x75, Opecode { name: Instruction::ADC, mode: Addressing::ZeroPageX, cycle: cycles[0x75] });
            m.insert(0x7D, Opecode { name: Instruction::ADC, mode: Addressing::AbsoluteX, cycle: cycles[0x7D] });
            m.insert(0x79, Opecode { name: Instruction::ADC, mode: Addressing::AbsoluteY, cycle: cycles[0x79] });
            m.insert(0x61, Opecode { name: Instruction::ADC, mode: Addressing::PreIndexedIndirect, cycle: cycles[0x61] });
            m.insert(0x71, Opecode { name: Instruction::ADC, mode: Addressing::PostIndexedIndirect, cycle: cycles[0x71] });
            m.insert(0xE9, Opecode { name: Instruction::SBC, mode: Addressing::Immediate, cycle: cycles[0xE9] });
            m.insert(0xE5, Opecode { name: Instruction::SBC, mode: Addressing::ZeroPage, cycle: cycles[0xE5] });
            m.insert(0xED, Opecode { name: Instruction::SBC, mode: Addressing::Absolute, cycle: cycles[0xED] });
            m.insert(0xF5, Opecode { name: Instruction::SBC, mode: Addressing::ZeroPageX, cycle: cycles[0xF5] });
            m.insert(0xFD, Opecode { name: Instruction::SBC, mode: Addressing::AbsoluteX, cycle: cycles[0xFD] });
            m.insert(0xF9, Opecode { name: Instruction::SBC, mode: Addressing::AbsoluteY, cycle: cycles[0xF9] });
            m.insert(0xE1, Opecode { name: Instruction::SBC, mode: Addressing::PreIndexedIndirect, cycle: cycles[0xE1] });
            m.insert(0xF1, Opecode { name: Instruction::SBC, mode: Addressing::PostIndexedIndirect, cycle: cycles[0xF1] });
            m.insert(0xE0, Opecode { name: Instruction::CPX, mode: Addressing::Immediate, cycle: cycles[0xE0] });
            m.insert(0xE4, Opecode { name: Instruction::CPX, mode: Addressing::ZeroPage, cycle: cycles[0xE4] });
            m.insert(0xEC, Opecode { name: Instruction::CPX, mode: Addressing::Absolute, cycle: cycles[0xEC] });
            m.insert(0xC0, Opecode { name: Instruction::CPY, mode: Addressing::Immediate, cycle: cycles[0xC0] });
            m.insert(0xC4, Opecode { name: Instruction::CPY, mode: Addressing::ZeroPage, cycle: cycles[0xC4] });
            m.insert(0xCC, Opecode { name: Instruction::CPY, mode: Addressing::Absolute, cycle: cycles[0xCC] });
            m.insert(0xC9, Opecode { name: Instruction::CMP, mode: Addressing::Immediate, cycle: cycles[0xC9] });
            m.insert(0xC5, Opecode { name: Instruction::CMP, mode: Addressing::ZeroPage, cycle: cycles[0xC5] });
            m.insert(0xCD, Opecode { name: Instruction::CMP, mode: Addressing::Absolute, cycle: cycles[0xCD] });
            m.insert(0xD5, Opecode { name: Instruction::CMP, mode: Addressing::ZeroPageX, cycle: cycles[0xD5] });
            m.insert(0xDD, Opecode { name: Instruction::CMP, mode: Addressing::AbsoluteX, cycle: cycles[0xDD] });
            m.insert(0xD9, Opecode { name: Instruction::CMP, mode: Addressing::AbsoluteY, cycle: cycles[0xD9] });
            m.insert(0xC1, Opecode { name: Instruction::CMP, mode: Addressing::PreIndexedIndirect, cycle: cycles[0xC1] });
            m.insert(0xD1, Opecode { name: Instruction::CMP, mode: Addressing::PostIndexedIndirect, cycle: cycles[0xD1] });
            m.insert(0x29, Opecode { name: Instruction::AND, mode: Addressing::Immediate, cycle: cycles[0x29] });
            m.insert(0x25, Opecode { name: Instruction::AND, mode: Addressing::ZeroPage, cycle: cycles[0x25] });
            m.insert(0x2D, Opecode { name: Instruction::AND, mode: Addressing::Absolute, cycle: cycles[0x2D] });
            m.insert(0x35, Opecode { name: Instruction::AND, mode: Addressing::ZeroPageX, cycle: cycles[0x35] });
            m.insert(0x3D, Opecode { name: Instruction::AND, mode: Addressing::AbsoluteX, cycle: cycles[0x3D] });
            m.insert(0x39, Opecode { name: Instruction::AND, mode: Addressing::AbsoluteY, cycle: cycles[0x39] });
            m.insert(0x21, Opecode { name: Instruction::AND, mode: Addressing::PreIndexedIndirect, cycle: cycles[0x21] });
            m.insert(0x31, Opecode { name: Instruction::AND, mode: Addressing::PostIndexedIndirect, cycle: cycles[0x31] });
 