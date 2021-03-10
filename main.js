import Oscillator from './src/nes/browser/oscillator.js';
import Noise from './src/nes/browser/noise.js';

let buf = null

const convertKeyCode = (keyCode) => {
  switch (keyCode) {
  case 88: return 0x01; // X  A
  case 90: return 0x02; // Z  B
  case 65: return 0x04; // A  SELECT
  case 83: return 0x08; // S  START
  case 38: return 0x10; // ↑  ↑
  case 40: return 0x20; // ↓  ↓
  case 37: return 0x40; // ←  ←
  case 39: return 0x80; // →  →
  }
};

const onKeydown = (e) => {
  if (buf != null)
    buf[buf.length - 1] |= convertKeyCode(e.keyCode);
}

const onKeyup = (e) => {
  if (buf != null)
    buf[buf.length - 1] &= ~convertKeyCode(e.keyCode);
}

const setupKeyHandler = () => {
  if (typeof window !== 'undefined') {
    document.addEventListener('keydown', onKeydown);
    document.addEventListener