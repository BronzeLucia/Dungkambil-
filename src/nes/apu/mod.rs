
mod constants;
mod noise;
mod square;
mod triangle;

use self::constants::*;
use self::noise::Noise;
use self::square::Square;
use self::triangle::Triangle;
use nes::types::{Addr, Data};

#[derive(Debug)]
pub struct Apu {
    squares: (Square, Square),
    triangle: Triangle,
    noise: Noise,
    cycle: u16,
    step: usize,
    sequencer_mode: bool,
    enable_irq: bool,
}

impl Apu {
    pub fn new() -> Self {
        Apu {
            squares: (Square::new(0), Square::new(1)),