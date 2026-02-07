//! level-mb2
//! ethan dibble <edibble@pdx.edu>

#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use microbit::{board::Board, display::blocking::Display, hal::timer::Timer};
use panic_halt as _;

#[rustfmt::skip]
const HEART: [[u8; 5]; 5] = [
    [0,1,0,1,0],
    [1,0,1,0,1],
    [1,0,0,0,1],
    [0,1,0,1,0],
    [0,0,1,0,0],
];

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    loop {
        display.show(&mut timer, HEART, 1000);
        timer.delay_ms(500);
        display.clear();
        timer.delay_ms(250);
    }
}
