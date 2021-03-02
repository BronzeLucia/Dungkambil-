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
$ np