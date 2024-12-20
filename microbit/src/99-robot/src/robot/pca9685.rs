pub const PCA9685_ADDRESS: u8 = 0x40;
pub const MODE1: u8     = 0x00;
pub const MODE2: u8     = 0x01;
pub const SUBADR1: u8   = 0x02;
pub const SUBADR2: u8   = 0x03;
pub const SUBADR3: u8   = 0x04;
pub const LED0_ON_L: u8 = 0x06;
pub const LED0_ON_H: u8 = 0x07;
pub const LED0_OFF_L: u8 = 0x08;
pub const LED0_OFF_H: u8 = 0x09;
pub const ALL_LED_ON_L: u8 = 0xFA;
pub const ALL_LED_ON_H: u8 = 0xFB;
pub const ALL_LED_OFF_L: u8 = 0xFC;
pub const ALL_LED_OFF_H: u8 = 0xFD;
pub const PRESCALE: u8  = 0xFE;

pub const STP_CHA_L: u16 = 2047;
pub const STP_CHA_H: u16 = 4095;

pub const STP_CHB_L: u16 = 1;
pub const STP_CHB_H: u16 = 2047;

pub const STP_CHC_L: u16 = 1023;
pub const STP_CHC_H: u16 = 3071;

pub const STP_CHD_L: u16 = 3071;
pub const STP_CHD_H: u16 = 1023;

pub enum Servos {
    S1 = 0x01,
    S2 = 0x02,
    S3 = 0x03,
    S4 = 0x04,
    S5 = 0x05,
    S6 = 0x06
}

#[derive(Copy, Clone)]
pub enum Motors {
    Left = 0x1,
    Right = 0x2,
    Centre = 0x3
}

pub enum Steppers {
    M1 = 0x1,
    M2 = 0x2
}

pub enum Turns {
    //% blockId="T1B4" block="1/4"
    T1B4 = 90,
    //% blockId="T1B2" block="1/2"
    T1B2 = 180,
    //% blockId="T1B0" block="1"
    T1B0 = 360,
    //% blockId="T2B0" block="2"
    T2B0 = 720,
    //% blockId="T3B0" block="3"
    T3B0 = 1080,
    //% blockId="T4B0" block="4"
    T4B0 = 1440,
    //% blockId="T5B0" block="5"
    T5B0 = 1800
}

