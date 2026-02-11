# level-mb2

A bubble level for the BBC Micro:Bit V2.

> Ethan Dibble  
> February 2026

## Dependencies

```
rustup target add thumbv7em-none-eabihf
rustup component add llvm-tools
cargo install cargo-binutils
cargo install --locked probe-rs-tools
```

## Build and Run

```
cargo embed
```

## How the level works

- The level will start in "coarse" mode;
- Pressing the B-button alone will swap to "fine" mode;
- Pressing the A-button alone will swap back to "coarse" mode;
- While the display is facing up, a single pixel will be lit depending on the
  board's orientation using the board's accelerometer updated every 200ms;
- While the display is facing the ground, the display will be turned off;
- While in "coarse" mode, the level will measure [-500,500];
- While in "fine" mode, the level will measure [-50,50].

## Notes

## License

This project is licensed under the [MIT License][License].

[License]: ./LICENSE
