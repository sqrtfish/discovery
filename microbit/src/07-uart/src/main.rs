#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

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

#[cfg(feature = "v1")]
use embedded_io::Write;

#[cfg(feature = "v2")]
use embedded_hal_nb::serial::Write;

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
        // Set up UART for microbit v1
        let serial = uart::Uart::new(
            board.UART0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        serial
    };

    #[cfg(feature = "v2")]
    let mut serial = {
        // Set up UARTE for microbit v2 using UartePort wrapper
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        // UartePort::new(serial)
    // };
    // static mut TX_BUF: [u8; 1] = [0; 1];
    static mut RX_BUF: [u8; 1] = [0; 1];

    // Write a byte and flush
    #[cfg(feature = "v1")]
    serial.write(&[b'X']).unwrap(); // Adjusted for UART on v1, no need for nb::block!

    #[cfg(feature = "v2")]
    {
        nb::block!(serial.write(b'X')).unwrap();
        nb::block!(serial.flush()).unwrap();
    }

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
        // nb::block!(serial.flush()).unwrap();

        loop {

            // We assume that the receiving cannot fail
            serial.read(unsafe {
                &mut RX_BUF
            }).unwrap();
            let byte = unsafe{RX_BUF[0]};
            rprintln!("{}", byte as char);
            // rprintln!("{}", b'\n');
            serial.write(unsafe{&mut RX_BUF}).unwrap();
            // nb::block!(serial.flush()).unwrap();
            // \r = 13, \n = 10
            if buffer.push(byte).is_err() {
                write!(serial, "error: buffer full\r\n").unwrap();
                break;
            }

            if byte == 13 {
                //nb::block!(serial.write(b'\n')).unwrap();
                write!(serial, "\nThe revers one is:\r\n").unwrap();
                // nb::block!(serial.flush()).unwrap();
                // for byte in buffer.iter().rev().chain(&[b'\n', b'\r']) {
                //     nb::block!(serial.write(byte)).unwrap();
                // }
                buffer = buffer.iter().rev().chain(&[b'\n', b'\r']).map(|&x| x).collect();
                serial.write(&buffer);
                break;
            }
        }
        // nb::block!(serial.flush()).unwrap()
    }
}
