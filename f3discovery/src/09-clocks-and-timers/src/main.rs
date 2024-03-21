#![no_main]
#![no_std]

use aux9::{entry, switch_hal::OutputSwitch, tim6};

#[inline(never)]
fn delay(_tim6: &tim6::RegisterBlock, ms: u16) {
    // TODO implement this
    // const ONE_MS: u32 = 72_000;
    //let delay = ms as u32 * ONE_MS;
    // for _ in 0..delay {
        // aux9::nop()
    // }
    _tim6.arr.write(|w| w.arr().bits(ms));
    _tim6.cr1.modify(|_, w| w.cen().set_bit());
    while !_tim6.sr.read().uif().bit_is_set() {}
    _tim6.sr.modify(|_, w| w.uif().clear_bit());
}

#[entry]
fn main() -> ! {
    let (leds, _rcc, tim6) = aux9::init();
    let mut leds = leds.into_array();
    let psc = 8000 - 1;

    // TODO initialize TIM6
    _rcc.apb1enr.modify(|_, w| w.tim6en().set_bit());
    tim6.cr1.write(|w| w.opm().set_bit().cen().clear_bit());
    tim6.psc.write(|w| w.psc().bits(psc));
    

    let ms = 50;
    loop {
        for curr in 0..8 {
            let next = (curr + 1) % 8;

            leds[next].on().unwrap();
            delay(tim6, ms);
            leds[curr].off().unwrap();
            delay(tim6, ms);
        }
    }
}
