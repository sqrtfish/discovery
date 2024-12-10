use core::fmt;
// use embedded_hal::blocking::serial as bserial;
// use embedded_hal::serial;
// use embedded_hal_nb::serial;
// use embedded_io;
// use embedded_hal_nb::serial::{Write, Read};
// use embedded_io_async::serial as bserial;
use microbit::hal::uarte::{Error, Instance, Uarte, UarteRx, UarteTx};
use nb;

static mut TX_BUF: [u8; 1] = [0; 1];
static mut RX_BUF: [u8; 1] = [0; 1];

pub struct UartePort<T: Instance>(UarteTx<T>, UarteRx<T>);

impl<T: Instance> UartePort<T> {
    pub fn new(serial: Uarte<T>) -> UartePort<T> {
        let (tx, rx) = serial
            .split(unsafe { &mut TX_BUF }, unsafe { &mut RX_BUF })
            .unwrap();
        UartePort(tx, rx)
    }
}

impl<T: Instance> fmt::Write for UartePort<T> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0.write_str(s)
    }
}

impl<T: Instance> embedded_io::ErrorType for UartePort<T> {
    type Error = Error;
}

impl<T: Instance> embedded_io::Write for UartePort<T> {
    // type Error = Error;

    fn write(&mut self, b: u8) -> nb::Result<(), Self::Error> {
        self.0.write(b)
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        self.0.flush()
    }
}

// impl<T: Instance> bserial::write::Default<u8> for UartePort<T> {}

impl<T: Instance> embedded_io::Read for UartePort<T> {
    // type Error = Error;

    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        self.0.read()
    }
}
