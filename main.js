import Oscillator from './src/nes/browser/oscillator.js';
import Noise from './src/nes/browser/noise.js';

let buf = null

const convertKeyCode = (keyCode) => {
  switch (keyCode) {
  case 88: return 0x01; // X  A
  case 90: return 0x02; // Z 