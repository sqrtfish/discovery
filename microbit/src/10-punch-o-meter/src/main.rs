#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print,};
use panic_rtt_target as _;

use microbit::{
    hal::{twim, timer},
    hal::delay::Delay,
    hal::prelude::*,
    pac::{twim0::frequency::FREQUENCY_A, timer0},
};

use lsm303agr::{
    AccelMode,
    AccelOutputDataRate,
    AccelScale,
    Lsm303agr,
};

use nb::Error;


#[entry]
fn main() -> ! {
    const ACCEL_THD: i32 = 500;
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();
    let i2c = { 
        twim::Twim::new(
            board.TWIM0, 
            board.i2c_internal.into(), 
            FREQUENCY_A::K100
        ) 
    };

    let mut delay = Delay::new(board.SYST);
    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor.set_accel_mode_and_odr(
        &mut delay, 
        AccelMode::HighResolution, 
        AccelOutputDataRate::Hz10
    ).unwrap();

    // let accel_as = AccelScale::G4;
    let mut timer = timer::Timer::new(board.TIMER0).into_periodic();

    sensor.set_accel_scale(AccelScale::G16).unwrap();
    loop {
        while !sensor.accel_status().unwrap().x_new_data() {}
        let data = sensor.acceleration().unwrap().x_mg();
        if data.abs() > ACCEL_THD {
            break;
        }
    }
    rprintln!("Start!");
//    rprintln!("current is { } mg", data);

    timer.start(1_000_000 as u32);
    //let mut result = timer.wait();
    let mut max_x: i32 = 0;

    loop {
        while !sensor.accel_status().unwrap().x_new_data() {};
        let data = sensor.acceleration().unwrap().x_mg();
        if data.abs() > max_x {
            max_x = data;
        }
        if timer.wait() == Ok(()) {
            rprintln!("max x accel is { } mg", max_x);
            max_x = 0;
        }
    }
}
