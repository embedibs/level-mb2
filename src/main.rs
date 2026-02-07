//! level-mb2
//! ethan dibble <edibble@pdx.edu>

#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use microbit::{
    display::blocking::Display,
    hal::{Timer, twim},
    pac::twim0::frequency::FREQUENCY_A,
};

use lsm303agr::{AccelMode, AccelOutputDataRate, Lsm303agr};

enum Mode {
    Coarse,
    Fine,
}

struct Point(i32, i32, i32);

impl Point {
    /// Create a new point reflected across the origin.
    pub fn new_inverted((x, y, z): (i32, i32, i32)) -> Self {
        Self(-x, -y, -z)
    }

    /// Return true if z-position is strictly positive.
    pub fn z_up(&self) -> bool {
        self.2 > 0
    }
}

/// 200ms or 5 frames per second.
const FRAME_TIME: u32 = 200;

/// 5x5 display buffer.
type Buf = [[u8; 5]; 5];

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let mut button_a = board.buttons.button_a;
    let mut button_b = board.buttons.button_b;

    let fb: &mut Buf = &mut Default::default();

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
        if sensor.accel_status().unwrap().xyz_new_data() {
            let p = Point::new_inverted(sensor.acceleration().unwrap().xyz_mg());
            if p.z_up() {
                rprintln!("Acceleration: x {} y {} z {}", p.0, p.1, p.2);
            }
        }

        // Update at a rate of FRAME_TIME.
        display.show(&mut timer, *fb, FRAME_TIME);
    }
}
