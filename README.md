# Rustynes

[![CircleCI](https://circleci.com/gh/bokuweb/rustynes.svg?style=svg)](https://circleci.com/gh/bokuweb/rustynes)

Work in progress

## Screenshot

<img src="https://github.com/bokuweb/flownes/blob/master/docs/screenshot.png?raw=true" />

## Demo

https://bokuweb.github.io/rustynes/

## requirements

- emscripten
- rust
- wasm-gc
- SDL2

### Using [Nix]

If you have the [Nix] package manager installed, you can alternatively run a nix shell with the necessary requirements automatically brought in scope:

``` sh
nix-shell
```

In that shell, you can then simply run the following commands.

[Nix]: https://nixos.org/nix/

## Development

### webAssembly
```
$ make
$ npm install
$ npm start
$ open http://localhost:3334
```

### Native

```
$ cargo run -p standalone --release roms/hello.nes
```

## Build

### webAssembly
```
$ make
```

### Native

```
$ make standalone
```

## Test

```
$cargo test
```

## TODO

- [ ] Mappers
  - [x] Mapper0
  - [ ] Mapper1
  - [ ] Mapper2
  - [x] Mapper3
  - [ ] Mapper4
  - [ ] Othres
- [x] PPU
  - [x] 8 * 16 Sprite
- [ ] APU
  - [x] Noise
  - [ ] DCM
- [ ] 2P GamePad  

### v0.1.0

prototype.

## License

The MIT License (MIT)

Copyright (c) 2018 @bokuweb

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

Th