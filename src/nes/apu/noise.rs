use super::constants::{NOISE_TIMER_PERIOD_TABLE, GROBAL_GAIN, CPU_CLOCK, COUNTER_TABLE};
use nes::types::{Data, Addr};

#[derive(Debug)]
pub struct Noise {
    envelope_generator_counter: usize,
    envelope_rate: usize,
    envelope_volume: usize,
    envelope_enable: bool,
    is_length_counter_enable: bool,
    length_counter: usize,

    divider_for_frequency: usize,
    frequency: usize,
    enable: bool,
}

extern "C