//#![deny(unsafe_code)]
#![no_main]
#![no_std]

use core::str;

use cortex_m_rt::entry;
// use nb::block;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;

// use microbit::hal::prelude::*;

#[cfg(feature = "v1")]
use microbit::{
    hal::twi,
    pac::twi0::frequency::FREQUENCY_A,
};

#[cfg(feature = "v2")]
use microbit::{
    hal::twim,
    pac::twim0::frequency::FREQUENCY_A,
};

use lsm303agr::{
    AccelMode,
    AccelOutputDataRate,
    MagMode,
    MagOutputDataRate,
    Lsm303agr
};
use microbit::hal::delay::Delay;

use core::fmt::Write;
use heapless::Vec;

#[cfg(feature = "v1")]
use microbit::{
    hal::prelude::*,
    hal::uart,
    hal::uart::{Baudrate, Parity},
};

#[cfg(feature = "v2")]
use microbit::{
    // hal::prelude::*,
    hal::uarte,
    hal::uarte::{Baudrate, Parity},
};

// #[cfg(feature = "v2")]
// mod serial_setup;
// #[cfg(feature = "v2")]
// use serial_setup::UartePort;

// const ACCELEROMETER_ADDR: u8 = 0b0011001;
// const MAGNETOMETER_ADDR: u8 = 0b0011110;

// const ACCELEROMETER_ID_REG: u8 = 0x0f;
// const MAGNETOMETER_ID_REG: u8 = 0x4f;

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
    // let mut serial = {
        let mut serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
    //     UartePort::new(serial)
    // };
    let mut rx_buff: [u8; 1] = [0; 1];

    // let mut acc = [0];
    // let mut mag = [0];

    // // First write the address + register onto the bus, then read the chip's responses
    // i2c.write_read(ACCELEROMETER_ADDR, &[ACCELEROMETER_ID_REG], &mut acc).unwrap();
    // i2c.write_read(MAGNETOMETER_ADDR, &[MAGNETOMETER_ID_REG], &mut mag).unwrap();

    // rprintln!("The accelerometer chip's id is: {:#b}", acc[0]);
    // rprintln!("The magnetometer chip's id is: {:#b}", mag[0]);

    // Code from crate lsm303agr

    let mut delay = Delay::new(board.SYST);
    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor.set_accel_mode_and_odr(&mut delay, AccelMode::Normal, AccelOutputDataRate::Hz50).unwrap();
    sensor.set_mag_mode_and_odr(&mut delay, MagMode::HighResolution, MagOutputDataRate::Hz50).unwrap();
    let mut sensor = sensor.into_mag_continuous().ok().unwrap();

    write!(serial, "CMDs: acc==Acceleration; mag==Magnetometer\r\nPlease input your cmd:").unwrap();
    // nb::block!(serial.flush()).unwrap();

    loop {
        let mut buffer: Vec<u8, 32> = Vec::new();

        loop {
            // let byte = block!(serial.read()).unwrap();
            serial.read(&mut rx_buff).unwrap();

            let byte = rx_buff[0];
            // rprintln!("{}", byte as char);
            // serial.write(&mut rx_buff).unwrap();

            if byte == 13 {
                // serial.write(&[b'\n']).unwrap();
                break;
            }

            if buffer.push(byte).is_err() {
                write!(serial,"error: buffer full!\r\n").unwrap();
                break;
            }
        }

        if str::from_utf8(&buffer).unwrap().trim() == "acc" {
            while !sensor.accel_status().unwrap().xyz_new_data() {}
            let data = sensor.acceleration().unwrap();
            // RTT instead of normal print
            rprintln!("Acceleration: x {:>5} y {:>5} z {:>5}", data.x_mg(), data.y_mg(), data.z_mg());
            write!(serial, "Acceleration: x {:>5} y {:>5} z {:>5}\r\n", data.x_mg(), data.y_mg(), data.z_mg()).unwrap();
        } else if str::from_utf8(&buffer).unwrap().trim() == "mag" {
            while !sensor.mag_status().unwrap().xyz_new_data() {}
            let data = sensor.magnetic_field().unwrap();
            rprintln!("Magnetometer: x {:>5} y {:>5} z {:>5}", data.x_nt(), data.y_nt(), data.z_nt());
            write!(serial, "Magnetometer: x {:>5} y {:>5} z {:>5}\r\n", data.x_nt(), data.y_nt(), data.z_nt()).unwrap();
        } else {
            write!(serial, "error: command not detected\r\n").unwrap();
        }

        // nb::block!(serial.flush()).unwrap();
        for _ in 0..200_000 {
            cortex_m::asm::nop();
        }
    }
}
