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

The acceleration is measured in mg (milli-g) so it generally lies in the range
[-1000,1000] on all axes when holding the board but could fall outside. The 
display is actually z-down, with the x-axis running from the b-button to the
a-button, and the y-axis running from the usb port to the pins. I decided to
store the reading as a point in space inverted over the origin to make it easier
to reason about. That way I could check in this new coordinate system if the
board was z-up and segment the measured region into pixels without flipping any
signs. Not necessary, but helpful for me.  

To segment the pixels, I clamped the reading into the half-open interval
[-max,max) of the level mode then divided the region by the width of a pixel and
relied on integer truncation to get indices in the set {0,1,2,3,4}. I could also
have used `f32::EPSILON` scaled by the max or even subtracted one on the upper
bound and done the whole thing with integer division, but I believe that board
has support for hardware floating point computations.  

Something maybe a bit dumb is wrapping the pixel coordinates in an Option type.
I just didn't want to have multiple places calling `display.show` or
`timer.delay`. It's necessary to delay on all paths to debounce the buttons.  

## License

This project is licensed under the [MIT License][License].

[License]: ./LICENSE
