#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;

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
    hal::prelude::*,
    hal::uarte,
    hal::uarte::{Baudrate, Parity},
};

#[cfg(feature = "v2")]
mod serial_setup;
#[cfg(feature = "v2")]
use serial_setup::UartePort;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

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

    // nb::block!(serial.write(b'X')).unwrap();
    
    // for byte in b"The quick brown fox jumps over the lazy dog.\r\n".iter() {
    //     nb::block!(serial.write(*byte)).unwrap();
    // }

    //write!(serial, "The quick brown fox jumps over the lazy dog.\r\n").unwrap();

    //nb::block!(serial.flush()).unwrap();

    // A buffer with 32 bytes of capacity
    let mut buffer: Vec<u8, 32> = Vec::new();

    loop {
        // let byte = nb::block!(serial.read()).unwrap();
        // rprintln!("{}", byte as char);
        // nb::block!(serial.write(byte)).unwrap();
        // nb::block!(serial.flush()).unwrap();

        buffer.clear();

        write!(serial, "Input the string, then enter:\r\n").unwrap();
        nb::block!(serial.flush()).unwrap();

        loop {

            // We assume that the receiving cannot fail
            let byte = nb::block!(serial.read()).unwrap();
            rprintln!("{}", byte as char);
            // rprintln!("{}", b'\n');
            nb::block!(serial.write(byte)).unwrap();
            nb::block!(serial.flush()).unwrap();
            // \r = 13, \n = 10
            if buffer.push(byte).is_err() {
                write!(serial, "error: buffer full\r\n").unwrap();
                break;
            }

            if byte == 13 {
                //nb::block!(serial.write(b'\n')).unwrap();
                write!(serial, "\nThe revers one is:\r\n").unwrap();
                nb::block!(serial.flush()).unwrap();
                for byte in buffer.iter().rev().chain(&[b'\n', b'\r']) {
                    nb::block!(serial.write(*byte)).unwrap();
                }
                break;
            }
        }
        nb::block!(serial.flush()).unwrap()
    }
}
