
import pulse from './pulse.js';

export default class Oscillator {

  constructor(type) {
    try {
      const AudioContext = window.AudioContext || window.webkitAudioContext
      this.context = new AudioContext();
    } catch (e) {
      throw new Error('Web Audio isn\'t supported in this browser!');
    }
    this.type = type || 'square';
    this.oscillator = this.createOscillator({ kind: this.type });

    this.waves = {
      '0.125': this.context.createPeriodicWave(pulse['0.125'].real, pulse['0.125'].imag),
      '0.25': this.context.createPeriodicWave(pulse['0.25'].real, pulse['0.25'].imag),
      '0.5': this.context.createPeriodicWave(pulse['0.5'].real, pulse['0.5'].imag),
      '0.75': this.context.createPeriodicWave(pulse['0.75'].real, pulse['0.75'].imag),
    };

    this.setVolume(0);
    this.setPulseWidth(0.5);
    this.playing = false;
  }

  start() {
    if (this.playing) {
      this.stop();
    }
    this.playing = true;
    this.oscillator.start(0);
  }

  stop() {
    if (this.playing) {
      this.setVolume(0);
      this.playing = false;
      this.oscillator.stop(this.context.currentTime);
      this.oscillator = this.createOscillator();
      this.setPulseWidth(0.5);
    }
  }

  close() {
    this.context.close();
  }

  createOscill