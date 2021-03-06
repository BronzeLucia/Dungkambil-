import {start} from './main.js';

const name = "wasm/rustynes";
window.Module = {
  preRun: [],
  postRun: [],
  wasmBinaryFile: `${name}.wasm`,
  noExitRuntime: true,
  print: text => {
    console.log(text);
  },
  printErr: text => {
    console.error(text);
  },
  onRuntimeInitialized() 