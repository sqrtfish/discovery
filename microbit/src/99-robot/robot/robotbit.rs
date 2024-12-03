// use microbit::gpio
use pwm_pca9685::{Address, Pca9685};
use microbit::{
    hal::twim,
    pac::twim0::frequency::FREQUENCY_A,
};

pub(crate) fn init_pca9685() {
    let board = microbit::Board::take().unwrap();   
    let address = Address::Default;

}