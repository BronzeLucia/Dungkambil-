
use super::constants::*;
use nes::types::{Data, Addr};

#[derive(Debug)]
pub struct Triangle {
    index: usize,
    is_length_counter_enable: bool,
    length_counter: usize,
    linear_counter: usize,
    divider_for_frequency: usize,
    frequency: usize,
    enable: bool,
    playing: bool,
}

extern "C" {
    fn start_oscillator(index: usize);
    fn stop_oscillator(index: usize);
    fn close_oscillator(index: usize);
    fn set_oscillator_frequency(index: usize, freq: usize);
    fn change_oscillator_frequency(index: usize, freq: usize);
    fn set_oscillator_volume(index: usize, volume: f32);
    fn set_oscillator_pulse_width(index: usize, width: f32);
}

impl Triangle {
    pub fn new(index: usize) -> Self {
        Triangle {
            index,
            is_length_counter_enable: false,
            length_counter: 0,
            linear_counter: 0,
            divider_for_frequency: 1,
            frequency: 0,
            enable: false,
            playing: false,
        }
    }

    fn get_volume(&self) -> f32 {
        32.0 / (16.0 / GROBAL_GAIN)
    }

    fn stop_oscillator(&mut self) {
        // self.length_counter = 0;
        // self.linear_counter = 0;
        unsafe {
            stop_oscillator(self.index);
            set_oscillator_volume(self.index, 0.0);
        };
    }
