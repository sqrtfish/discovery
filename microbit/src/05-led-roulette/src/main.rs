#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
// use panic_halt as _;
use panic_rtt_target as _;
// use microbit as _;
// use microbit::board::Board;
// use microbit::hal::timer::Timer;
// use microbit::hal::prelude::*;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::Timer,
};

const PIXELS: [(usize, usize); 16] = [
    (0,0), (0,1), (0,2), (0,3), (0,4), (1,4), (2,4), (3,4), (4,4),
    (4,3), (4,2), (4,1), (4,0), (3,0), (2,0), (1,0)
];

#[entry]
fn main() -> ! {
    // let _y;
    // let x = 42;
    // _y = x;
    rtt_init_print!();
    let board = Board::take().unwrap();

    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut leds = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];

    let mut last_led = (0,0);

    // let light_it_all = [
    //     [1, 1, 1, 1, 1],
    //     [1, 1, 1, 1, 1],
    //     [1, 1, 1, 1, 1],
    //     [1, 1, 1, 1, 1],
    //     [1, 1, 1, 1, 1],
    // ];

    // board.display_pins.col1.set_low().unwrap();
    // let mut row1 = board.display_pins.row1;
    // rprintln!("light all leds on!");
    rprintln!("leds run!");

    // infinite loop; just so we don't leave this stack frame
    loop {
        // row1.set_low().unwrap();
        // rprintln!("Dark!");
        // timer.delay_ms(1_000_u16);
        // row1.set_high().unwrap();
        // rprintln!("Light!");
        // timer.delay_ms(1_000_u16);
        // timer.delay_ms(1000u16);
        // rprintln!("1000 ms passed");
        
        // Show light_it_all for 1000ms
        //display.show(&mut timer, light_it_all, 1000);
        // clear the display again
        //display.clear();
        //timer.delay_ms(1000_u32);
        
        for current_led in PIXELS.iter() {
            leds[last_led.0][last_led.1] = 0;
            leds[current_led.0][current_led.1] = 1;
            display.show(&mut timer, leds, 30);
            last_led = *current_led;
        }

    }
}
