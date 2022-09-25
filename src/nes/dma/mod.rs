use super::types::{Data};
use nes::ram::Ram;
use nes::ppu::Ppu;

#[derive(Debug)]
pub struct Dma {
    register: Data,
    should_run: bool,
}

impl Dma {
    pub fn new() -> Self {
        Dma {
            register: 0,
            should_run: false,
        }
    }

    pub