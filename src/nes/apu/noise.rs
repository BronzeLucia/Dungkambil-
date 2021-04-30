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

extern "C" {
    fn set_noise_frequency(freq: f32);
    fn set_noise_volume(volume: f32);
    fn stop_noise();
    fn start_noise();
// fn close_noise();
}

impl Noise {
    pub fn new() -> Self {
        Noise {
            envelope_generator_counter: 0,
            envelope_rate: 0x0F,
            envelope_volume: 0x0F,
            envelope_enable: false,

            is_length_counter_enable: false,
            length_counter: 0,
            divider_for_frequency: 1,
            frequency: 0,
            enable: false,
        }
    }

    pub fn enable(&mut self) {
        self.enable = true;
        self.start();
    }

    pub fn disable(&mut self) {
        self.enable = false;
        self.stop();
    }


    fn get_volume(&self) -> f32 {
        let vol = if self.envelope_enable {
            self.envelope_volume
        } else {
            self.envelope_rate
        };
        vol as f32 / (16.0 / GROBAL_GAIN)
    }

    pub fn update_envelope(&mut self) {
        self.envelope_generator_counter -= 1;
        if self.envelope_generator_counter <= 0 {
            self.envelope_generator_counter = self.envelope_rate;
            if self.envelope_volume > 0 {
                self.envelope_volume -= 1;
            } else {
                self.stop();
                self.envelope_volume = 0x0F;
            }
        }
        self.set_volume();
    }

    pub fn 