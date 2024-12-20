use core::fmt::Debug;
use embedded_hal::i2c::I2c;
use embedded_hal::delay::DelayNs;
use nb::Error;
use rtt_target::rprintln;

use crate::robot::pca9685::PRESCALE;

use super::tm1650;
// use crate::pca9685;

use super::pca9685::{
    Servos, Motors, 
    LED0_ON_L, MODE1, PCA9685_ADDRESS, 
    STP_CHA_H, STP_CHA_L, STP_CHB_H, 
    STP_CHB_L, STP_CHC_H, STP_CHC_L, 
    STP_CHD_H, STP_CHD_L};

pub enum Obstacle {
    //% blockId="Obstacle" block="有障碍物"
    Obstacle = 0,
    //% blockId="NoObstacle" block="无障碍物"
    NoObstacle = 1
}

pub enum Flame {
    //% blockId="Flame" block="发现火焰"
    Flame = 0,
    //% blockId="NoFlame" block="无火焰"
    NoFlame = 1
}

pub enum PingUnit{
    //% block="us"
    MicroSeconds,
    //% block="cm"
    Centimeters,
    //% block="inches"
    Inches
}

// #[derive(Debug, Default, Clone, Copy, PartialEq)]
#[derive(Debug)]
pub struct Motor {
    pub motor_speed: i16,
    pub motor_freq: u8,
    pub motor_run_seconds: u32,
}

// #[derive(Debug, Default, Clone, Copy, PartialEq)]
#[derive(Debug)]
pub struct Seg4 {
    pub intensity: u8,
    pub cmd: [u8; 1],
    pub dbuf: [u8; 4],
}

// #[derive(Debug, Default, Clone, Copy, PartialEq)]
#[derive(Debug)]
pub struct Robotbit<I2C, T> {
    i2c: I2C,
    timer: T,
    motors: Motor,
    seg4: Seg4,
}

impl<I2C, T, ERROR> Robotbit<I2C, T> 
where 
    I2C: I2c<Error = ERROR>,
    T: DelayNs,
{
    pub fn default(i2c: I2C, timer: T) -> Result<Self, Error<ERROR>> {
        Ok(Robotbit {
            i2c,
            timer,
            motors: Motor { 
                motor_speed: 150, 
                motor_freq: 50,
                motor_run_seconds: 1,
            },
            seg4: Seg4 { intensity: 3, cmd: [49], dbuf: [1, 1, 0, 2]}
        })
    }

    pub fn destroy(self) -> (I2C, T) {
        (self.i2c, self.timer)
    }

    pub fn display(&mut self) {
        // while !self.i2c.write(tm1650::CMD_ADDR, &self.seg4.cmd).is_ok() {};
        for i in 0..4 {
            self.seg4.cmd[0] = tm1650::SEG[self.seg4.dbuf[i] as usize];
            // rprintln!("{:?}",seg4);
            while !self.i2c.write(tm1650::DISPLAY_ADDR[i],
                &self.seg4.cmd).is_ok() {};
        }
    }

    pub fn init(&mut self)
    {
        while !self.i2c.write(PCA9685_ADDRESS,&[MODE1, 0x00]).is_ok() {};
        // self.timer.delay_ms(1);
        self.set_freq(self.motors.motor_freq as f32);
        for ch in 0..16 {
            self.set_pwm(ch, 0, 0);
        }
    }

    pub fn init_display(&mut self) {
        while !self.i2c.write(tm1650::CMD_ADDR, &self.seg4.cmd).is_ok() {};
    }

    fn set_freq(&mut self, freq: f32) 
    // where 
    //     T: DelayNs,
    {
        let mut prescalerval = 25000000.0;
        prescalerval /= 4096.0;
        prescalerval /= freq;
        prescalerval -= 1.0;
        rprintln!("prescalerval = {:?}", prescalerval);
        let mut oldmode:[u8;1] = [0];
        while !self.i2c.write_read(PCA9685_ADDRESS, 
            &[MODE1], 
            &mut oldmode[0..1]).is_ok() {};
        // self.timer.delay_ms(1);
        rprintln!("oldmode = {:?}", oldmode);
        let newmode = (oldmode[0] & 0x7F) | 0x10;
        rprintln!("newmode = {:?}", newmode);
        rprintln!("oldmode = {:?}", oldmode);
        while !self.i2c.write(PCA9685_ADDRESS, &[MODE1, newmode]).is_ok() {};
        // self.timer.delay_ms(1);
        while !self.i2c.write(PCA9685_ADDRESS, &[PRESCALE, prescalerval as u8]).is_ok() {};
        // self.timer.delay_ms(1);
        while !self.i2c.write(PCA9685_ADDRESS, &[MODE1, oldmode[0]]).is_ok() {};
        rprintln!("oldmode = {:?}", oldmode);
        self.timer.delay_ms(5000);
        while !self.i2c.write(PCA9685_ADDRESS, &[MODE1, oldmode[0] | 0xA1]).is_ok() {};
        // self.timer.delay_ms(1);
    }

    fn set_pwm (&mut self, ch: u8, on: u16, off: u16)
    {
        // if ch >= 16 {
        //     return;
        // }

        let buf = [
            LED0_ON_L + 4 * ch,
            (on & 0xFF) as u8,
            ((on >> 8) & 0xFF) as u8,
            (off & 0xFF) as u8,
            ((off >> 8) & 0xFF) as u8
        ];
        rprintln!("buf = {:?}", buf);

        while !self.i2c.write(PCA9685_ADDRESS, &buf).is_ok() {};
        // self.timer.delay_ms(1);
    }

    fn set_level(&mut self, servos: Servos, value: bool) {
        if value {
            self.set_pwm(servos as u8 + 7, 0, 4095);
        } else {
            self.set_pwm(servos as u8 + 7, 0, 0);
        }
    }

    fn set_stepper(&mut self, index: u8, direction: bool) {
        if index == 1 {
            if direction {
                self.set_pwm(0, STP_CHA_L, STP_CHA_H);
                self.set_pwm(2, STP_CHB_L, STP_CHB_H);
                self.set_pwm(1, STP_CHC_L, STP_CHC_H);
                self.set_pwm(3, STP_CHD_L, STP_CHD_H);
            } else {
                self.set_pwm(3, STP_CHA_L, STP_CHA_H);
                self.set_pwm(1, STP_CHB_L, STP_CHB_H);
                self.set_pwm(2, STP_CHC_L, STP_CHC_H);
                self.set_pwm(0, STP_CHD_L, STP_CHD_H);
            }
        } else {
            if direction {
                self.set_pwm(4, STP_CHA_L, STP_CHA_H);
                self.set_pwm(6, STP_CHB_L, STP_CHB_H);
                self.set_pwm(5, STP_CHC_L, STP_CHC_H);
                self.set_pwm(7, STP_CHD_L, STP_CHD_H);
            } else {
                self.set_pwm(7, STP_CHA_L, STP_CHA_H);
                self.set_pwm(5, STP_CHB_L, STP_CHB_H);
                self.set_pwm(6, STP_CHC_L, STP_CHC_H);
                self.set_pwm(4, STP_CHD_L, STP_CHD_H);
            }
        }
    }

    fn stop_motor(&mut self, motor: Motors) {
        let ch = (motor as u8 - 1) * 2;
        self.set_pwm(ch, 0, 0);
        self.set_pwm(ch + 1, 0, 0);
    }

    fn set_servo(&mut self, servo: Servos, degree: f32) {
        let v_us = degree * 10.0 + 600.0;
        let value = (v_us * 4096.0 / 20000.0) as u16;
        self.set_pwm(servo as u8 + 7, 0, value);
    }

    fn set_motor(&mut self, motor: Motors, speed: i16) {
        let mut speed = speed * 16;
        if speed > 4095 {
            speed = 4095;
        } else if speed < -4095 {
            speed = -4095;
        }

        let pp = (motor as u8 - 1) * 2;
        let pn = pp + 1;

        if speed > 0 {
            self.set_pwm(pp, 0, speed as u16);
            self.set_pwm(pn, 0, 0);
        } else {
            speed = 0 - speed;
            self.set_pwm(pp, 0, 0);
            self.set_pwm(pn, 0, speed as u16);
        }
    }

    fn run_two_motors (&mut self, 
        motor1: Motors, speed1: i16, 
        motor2: Motors, speed2: i16) {
        self.set_motor(motor1, speed1);
        self.set_motor(motor2, speed2);
    }

    fn run_motor_for_seconds (&mut self, 
        motor: Motors, speed: i16) 
    {
        self.set_motor(motor, speed);
        self.timer.delay_ms(self.motors.motor_run_seconds * 1000);
        self.set_motor(motor, 0);
    }
    
    pub fn stop_all_motors (&mut self) {
        self.stop_motor(Motors::Left);
        self.stop_motor(Motors::Right);
        self.stop_motor(Motors::Centre); 
    }

    pub fn run_fan (&mut self) {
        self.set_motor(Motors::Centre, self.motors.motor_speed);
    }

    pub fn stop_fan(&mut self) {
        self.stop_motor(Motors::Centre);
    }

    pub fn forward(&mut self) {
        self.run_two_motors(
            Motors::Left, self.motors.motor_speed, 
            Motors::Right, self.motors.motor_speed);
    }

    pub fn backward(&mut self) {
        self.run_two_motors(
            Motors::Left, 0 - self.motors.motor_speed, 
            Motors::Right, 0 - self.motors.motor_speed);
    }

    pub fn stop(&mut self) {
        self.run_two_motors(
            Motors::Left, 0, 
            Motors::Right, 0);
    }

    pub fn turn_right_at_place(&mut self) {
        self.run_two_motors(
            Motors::Left, self.motors.motor_speed, 
            Motors::Right, 0 - self.motors.motor_speed);
    }

    pub fn turn_left_at_place(&mut self) {
        self.run_two_motors(
            Motors::Left, 0 - self.motors.motor_speed, 
            Motors::Right, self.motors.motor_speed);
    }

    pub fn turn_left(&mut self) {
        self.run_two_motors(
            Motors::Left, 0, 
            Motors::Right, 50 + self.motors.motor_speed);
    }

    pub fn turn_right(&mut self) {
        self.run_two_motors(
            Motors::Left, 50 + self.motors.motor_speed, 
            Motors::Right, 0);
    }

    pub fn turn_head(&mut self, degree: f32) {
        self.set_servo(Servos::S1, degree);
    }

    pub fn delay (&mut self)
    // where T: DelayNs,
    {
        self.timer.delay_ms(self.motors.motor_run_seconds * 1000);
    }

    pub fn delay_s (&mut self, seconds: u32)
    // where T: DelayNs,
    {
        self.timer.delay_ms(seconds * 1000);
    }

    pub fn update_seg4(&mut self, dbuf: &[u8]) {
        for i in 0..4 {
            self.seg4.dbuf[i] = dbuf[i];
        }
    }
}