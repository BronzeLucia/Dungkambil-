
import pulse from './pulse.js';

export default class Oscillator {

  constructor(type) {
    try {
      const AudioContext = window.AudioContext || window.webkitAudioContext
      this.context = new AudioContext();
    } ca