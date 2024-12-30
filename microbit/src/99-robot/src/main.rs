#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::digital::InputPin;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;
// use panic_halt as _;

use core::fmt::Write;
use heapless::Vec;

use microbit::{
    display::blocking::Display, gpio::BTN_A, hal::{
        delay::Delay, gpio::{Input, PullUp, p0::P0_14}, twim, uarte::{self, Baudrate, Parity}, Timer
    }, pac::twim0::frequency::FREQUENCY_A,
};


use lsm303agr::{
    AccelMode,
    AccelOutputDataRate,
    Lsm303agr,
    MagMode,
    MagOutputDataRate,
};

// use pwm_pca9685;

mod robot;
use crate::robot::robotbit;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();
    
    // let i2c = twim::Twim::new(
    //     board.TWIM0, 
    //     board.i2c_internal.into(),
    //     FREQUENCY_A::K100,
    // );

    let i2c_e = twim::Twim::new(
        board.TWIM0,
        board.i2c_external.into(),
        FREQUENCY_A::K100,
    );

    // let mut i2c = i2c_e;



    let mut serial = uarte::Uarte::new(
        board.UARTE0,
        board.uart.into(),
        Parity::EXCLUDED,
        Baudrate::BAUD115200,
    );

    let mut timer = Timer::new(board.TIMER0);

    let mut display = Display::new(board.display_pins);

    let delay = Delay::new(board.SYST);

    let echo = board.edge.e00.into_floating_input();

    // let sensor_lsm303 = Lsm303agr::new_with_i2c(i2c);

    // let senser_pca9685 = pwm_pca9685::Pca9685::new(i2c_e, 0x40).unwrap();
    
    let mut robot = robotbit::Robotbit::default(i2c_e, delay, echo).unwrap();
    
    // let mut gpiote = Gpiote::new(board.GPIOTE);
    let mut button_a = board.buttons.button_a.into_pullup_input();

    robot.init();
    // rprintln!("Robot initialized");
    write!(serial, "Robot initialized\r\n").unwrap();
    // pause(&mut button_a);
    // robot.display();
    // rprintln!("Robot display");
    robot.delay();
    robot.stop();
    // rprintln!("Robot stopped");
    write!(serial, "Robot stopped\r\n").unwrap();
    // pause(&mut button_a);
    robot.delay();
    // robot.forward();
    // rprintln!("Robot forward");
    write!(serial, "Robot forward\r\n").unwrap();
    // pause(&mut button_a);
    robot.delay();
    // robot.backward();
    // rprintln!("Robot backward");
    write!(serial, "Robot backward\r\n").unwrap();
    // pause(&mut button_a);
    robot.delay();
    robot.stop();
    robot.delay();
    robot.turn_head(0.0);
    // rprintln!("Robot turn head 0");
    write!(serial, "Robot turn head 0\r\n").unwrap();
    // pause(&mut button_a);
    robot.delay();
    robot.turn_head(180.0);
    // rprintln!("Robot turn head 180");
    write!(serial, "Robot turn head 180\r\n").unwrap();
    // pause(&mut button_a);
    robot.delay();
    robot.turn_head(90.0);
    robot.delay();
    write!(serial, "Robot turn head 90\r\n").unwrap();
    robot.init_display();
    robot.display();
    rprintln!("Robot display");
    // robot.run_fan();
    write!(serial, "Robot run fan\r\n").unwrap();
    robot.delay();
    robot.stop_fan();
    write!(serial, "Robot stop fan\r\n").unwrap();
    // pause(&mut button_a);

    loop {
        for i in 0..10 {
            robot.update_seg4(&[i, i + 1, i + 2, i + 3]);
            robot.display();
            // robot.turn_head(i as f32 * 20.0);
            robot.delay();
            write!(serial, "echo is {}\r\n", robot.hcsr04_echo()).unwrap();
        }
        // robot.delay();
    }


}

fn pause(button_a: &mut P0_14<Input<PullUp>>) 
// where BTN: P0_14<Input<PullUp>>,
{   
    rprintln!("Paused!");
    while button_a.is_high().unwrap() {
        // rprintln!("Waiting for button press {:?}", button_a.is_low());
    };
    rprintln!("{:?}", button_a.is_low());
    rprintln!("{:?}", button_a.is_high());
    rprintln!("Resumed!");
}