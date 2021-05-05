
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

    this.wa