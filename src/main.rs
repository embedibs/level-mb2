//! level-mb2
//! ethan dibble <edibble@pdx.edu>

#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::InputPin};
use panic_rtt_target as _;

use microbit::{
    display::blocking::Display,
    hal::{Timer, twim},
    pac::twim0::frequency::FREQUENCY_A,
};

use lsm303agr::{AccelMode, AccelOutputDataRate, Lsm303agr};

#[derive(core::fmt::Debug)]
enum LevelMode {
    /// Clamp display to [-500,500]
    Coarse,
    /// Clamp display to [-50,50]
    Fine,
}

#[derive(core::fmt::Debug, Default)]
struct Point(i32, i32, i32);

impl Point {
    /// Create a new point reflected across the origin.
    fn new_inverted((x, y, z): (i32, i32, i32)) -> Self {
        Point(-x, -y, -z)
    }

    /// Return true if z-position is strictly positive.
    fn z_up(&self) -> bool {
        self.2 > 0
    }

    /// Consume self and return pixel location.
    fn to_pixel(self, mode: &LevelMode) -> (usize, usize) {
        let Point(x, y, _) = self;
        let (max, step) = match mode {
            LevelMode::Coarse => (500, 200.1),
            LevelMode::Fine => (50, 20.1),
        };

        let (x, y) = (x.clamp(-max, max) + max, y.clamp(-max, max) + max);

        ((x as f32 / step) as usize, 4 - (y as f32 / step) as usize)
    }
}

/// 200ms or 5 frames per second.
const FRAME_TIME: u32 = 200;

/// 5x5 display buffer.
type Buf = [[u8; 5]; 5];

#[entry]
fn main() -> ! {
    let board = microbit::Board::take().unwrap();

    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let mut button_a = board.buttons.button_a;
    let mut button_b = board.buttons.button_b;

    let fb: &mut Buf = &mut Default::default();
    let mut mode = LevelMode::Coarse;

    #[rustfmt::skip]
    let i2c = twim::Twim::new(
        board.TWIM0,
        board.i2c_internal.into(),
        FREQUENCY_A::K100);

    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor
        .set_accel_mode_and_odr(
            &mut timer,
            AccelMode::HighResolution,
            AccelOutputDataRate::Hz50,
        )
        .unwrap();

    loop {
        // poll button presses.
        let a_pressed = button_a.is_low().unwrap();
        let b_pressed = button_b.is_low().unwrap();

        mode = match (a_pressed, b_pressed) {
            (true, false) => LevelMode::Coarse,
            (false, true) => LevelMode::Fine,
            _ => mode,
        };

        let p: Point = if sensor.accel_status().unwrap().xyz_new_data() {
            Point::new_inverted(sensor.acceleration().unwrap().xyz_mg())
        } else {
            continue;
        };

        // Update bubble level at a rate of FRAME_TIME.
        if p.z_up() {
            let (x, y) = p.to_pixel(&mode);

            fb[y][x] = 1u8;
            display.show(&mut timer, *fb, FRAME_TIME);
            fb[y][x] = 0u8;
        } else {
            // The display will clear itself after 200ms so that pixels have no
            // chance of getting stuck on while the display is facing the ground.
            timer.delay_ms(FRAME_TIME);
        }
    }
}
