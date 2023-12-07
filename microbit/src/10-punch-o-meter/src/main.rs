#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print,};
use panic_rtt_target as _;

use microbit::{
    hal::twim,
    hal::delay::Delay,
    pac::twim0::frequency::FREQUENCY_A,
};

use lsm303agr::{
    AccelMode,
    AccelOutputDataRate,
    Lsm303agr,
};

#[entry]
fn main() -> ! {
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

    loop {
        while !sensor.accel_status().unwrap().xyz_new_data() {};
        let data = sensor.acceleration().unwrap();
        rprintln!(
            "Acceleration : x { } y { } z { }",
            data.x_mg(),
            data.y_mg(),
            data.z_mg()
        );
    }
}
