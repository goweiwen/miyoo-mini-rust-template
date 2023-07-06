# Miyoo Mini Rust Template

## About

This repository is an example of how to write a Rust program that can read inputs and draw to the screen on the Miyoo Mini. 

For graphics, we use [embedded_graphics](https://docs.rs/embedded-graphics/latest/embedded_graphics/) for drawing to a framebuffer, which we then use blit to the device's framebuffer.

For inputs, we use [evdev](https://docs.rs/evdev/latest/evdev/) to poll for inputs.

## Building

### Development

### Requirements
1. `cargo`
2. [SDL2](https://github.com/Rust-SDL2/rust-sdl2#sdl20-development-libraries) (optional, if simulator is not used)
3. [cross](https://github.com/cross-rs/cross): `cargo install cross --git https://github.com/cross-rs/cross` (optional, for cross-compilation)


### Simulator
For development work, there is a simulator that requires SDL2 to be installed.
```
cargo run
```

### Miyoo Mini
We use [cross](https://github.com/cross-rs/cross) for cross-compilation to the `arm-unknown-gnueabihf` target for the Miyoo Mini.
```
cross build --release
cp -r static/. dist
cp target/arm-unknown-linux-gnueabihf/release/miyoo-mini-rust-template "dist/Rust App Example.pak/"
```