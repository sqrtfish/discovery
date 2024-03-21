#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, Delay, DelayMs, LedArray, OutputSwitch};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds):(Delay, LedArray) = aux5::init();
    let half_period = 50_u16;
    let mut i = 0_u8;

    // infinite loop; just so we don't leave this stack frame
    loop {
        match i {
            0 => leds[0].on().ok(),
            2 => leds[1].on().ok(),
            4 => leds[2].on().ok(),
            6 => leds[3].on().ok(),
            8 => leds[4].on().ok(),
            10 => leds[5].on().ok(),
            12 => leds[6].on().ok(),
            14 => leds[7].on().ok(),
            1 => leds[7].off().ok(),
            3 => leds[0].off().ok(),
            5 => leds[1].off().ok(),
            7 => leds[2].off().ok(),
            9 => leds[3].off().ok(),
            11 => leds[4].off().ok(),
            13 => leds[5].off().ok(),
            15 => leds[6].off().ok(),
            _ => None,
        };
        
        if i == 15 {
            i = 0;
        } else {
            i += 1;
        }

        delay.delay_ms(half_period);
    }
}
