
use super::constants::*;
use nes::types::{Data, Addr};

#[derive(Debug)]
pub struct Square {
    index: usize,
    sweep_unit_counter: usize,
    length_counter: usize,
    is_length_counter_enable: bool,
    sweep_unit_divider: usize,
    frequency: usize,
    sweep_shift_amount: usize,
    is_sweep_enabled: bool,
    sweep_mode: bool,
    divider_for_frequency: usize,
    envelope_loop_enable: bool,
    envelope_generator_counter: usize,
    envelope_rate: usize,
    envelope_volume: usize,
    envelope_enable: bool,
    enable: bool,
    playing: bool,
}

extern "C" {
    fn start_oscillator(index: usize);
    fn stop_oscillator(index: usize);
    // fn close_oscillator(index: usize);
    fn set_oscillator_frequency(index: usize, freq: usize);
    fn change_oscillator_frequency(index: usize, freq: usize);
    fn set_oscillator_volume(index: usize, volume: f32);
    fn set_oscillator_pulse_width(index: usize, width: f32);
}

impl Square {
    pub fn new(index: usize) -> Self {
        Square {
            index,
            sweep_unit_counter: 0,
            length_counter: 0,
            sweep_unit_divider: 1,
            frequency: 0,
            sweep_shift_amount: 0,
            is_sweep_enabled: false,
            sweep_mode: false,
            divider_for_frequency: 1,
            envelope_loop_enable: false,
            envelope_generator_counter: 0,
            envelope_rate: 0x0F,
            envelope_volume: 0,
            envelope_enable: false,
            is_length_counter_enable: false,
            enable: false,
            playing: false,
        }
    }

    fn get_volume(&self) -> f32 {
        let vol = if self.envelope_enable {
            self.envelope_volume
        } else {
            self.envelope_rate
        };
        vol as f32 / (GROBAL_GAIN)
    }

    // fn stop_oscillator(&mut self) {
    //     unsafe {
    //         stop_oscillator(self.index);
    //     };
    // }

    // Length counter
    // When clocked by the frame counter, the length counter is decremented except when:
    // The length counter is 0, or The halt flag is set
    pub fn update_counters(&mut self) {
        if self.is_length_counter_enable && self.length_counter > 0 {
            self.length_counter -= 1;
            if self.length_counter == 0 {
                self.stop();
            }
        }

        if !self.is_sweep_enabled || !self.playing {
            return;
        };

        self.sweep_unit_counter += 1;
        if self.sweep_unit_counter % self.sweep_unit_divider == 0 {
            self.sweep_unit_counter = 0;
            // INFO:
            // sweep mode 0 : newPeriod = currentPeriod - (currentPeriod >> N)
            // sweep mode 1 : newPeriod = currentPeriod + (currentPeriod >> N)
            if self.sweep_mode {
                self.divider_for_frequency = self.divider_for_frequency -
                                             (self.divider_for_frequency >>
                                              self.sweep_shift_amount);
            } else {