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

    pub fn write(&mut self, data: Data) {
        self.register = data;
        self.should_run = true;
    }

    pub fn should_run(&self) -> bool {
        self.should_run
    }    

    pub fn run(&mut self, ram