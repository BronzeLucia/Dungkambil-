


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
    PHA