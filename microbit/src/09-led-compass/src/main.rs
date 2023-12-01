// #![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_rtt_target as _;
//use rtt_target::{rprintln, rtt_init_print};
use rtt_target::rtt_init_print;

mod calibration;
use crate::calibration::calc_calibration;
use crate::calibration::calibrated_measurement;
// use calibration::Measurement;
use crate::calibration::convert_tuple_to_measurement;

use microbit::{display::blocking::Display, hal::Timer};

#[cfg(feature = "v1")]
use microbit::{hal::twi, pac::twi0::frequency::FREQUENCY_A};

#[cfg(feature = "v2")]
use microbit::{hal::twim, pac::twim0::frequency::FREQUENCY_A};

use lsm303agr::{
    AccelMode,
    AccelOutputDataRate, 
    Lsm303agr,
    MagMode, 
    MagOutputDataRate
};

use microbit::hal::delay::Delay;

use nb;
use core::fmt::Write;

#[cfg(feature = "v1")]
use microbit::{
    hal::prelude::*,
    hal::uart,
    hal::uart::{Baudrate, Parity},
};

#[cfg(feature = "v2")]
use microbit::{
    hal::prelude::*,
    hal::uarte,
    hal::uarte::{Baudrate, Parity},
};

#[cfg(feature = "v2")]
mod serial_setup;
#[cfg(feature = "v2")]
use serial_setup::UartePort;

mod led;
use led::Direction;
use crate::led::direction_to_led;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    #[cfg(feature = "v1")]
    let i2c = { twi::Twi::new(board.TWI0, board.i2c.into(), FREQUENCY_A::K100) };

    #[cfg(feature = "v2")]
    let i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };

    #[cfg(feature = "v1")]
    let mut serial = {
        uart::Uart::new(
            board.UART0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        )
    };

    #[cfg(feature = "v2")]
    let mut serial = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };

    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let mut delay = Delay::new(board.SYST);

    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor.set_mag_mode_and_odr(&mut delay, MagMode::HighResolution, MagOutputDataRate::Hz10).unwrap();
    sensor.set_accel_mode_and_odr(&mut delay, AccelMode::HighResolution, AccelOutputDataRate::Hz10).unwrap();
    let mut sensor = sensor.into_mag_continuous().ok().unwrap();

    let calibration = calc_calibration(&mut sensor, &mut display, &mut timer);
    // rprintln!("Calibration: {:?}", calibration);
    // rprintln!("Calibration done, entering busy loop");
    
    write!(serial, "Calibration: {:?}\r\n", calibration).unwrap();
    write!(serial,"Calibration done, entering busy loop.\r\n").unwrap();

    loop {
        while !sensor.mag_status().unwrap().xyz_new_data() {}
        let mut data = convert_tuple_to_measurement(sensor.magnetic_field().unwrap().xyz_nt());
        data = calibrated_measurement(data, &calibration);
        //rprintln!("x: {}, y: {}, z: {}", data.x, data.y, data.z);
        write!(serial, "x: {}, y: {}, z: {}\r\n", data.x, data.y, data.z).unwrap();
        nb::block!(serial.flush()).unwrap();

        let dir = match (data.x > 0, data.y > 0) {
            //
            (true, true) => Direction::East,
            //
            (false, true) => Direction::North,
            //
            (false, false) => Direction::West,
            //
            (true, false) => Direction::South,
        };

        display.show(&mut timer, direction_to_led(dir), 1000);
    }
}
