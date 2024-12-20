
// use core::fmt::Debug;
// use microbit::{
//     hal::twim::{self, Instance},
//     pac::{
//         self,
//         twim0::frequency::FREQUENCY_A,
//         TWIM0,
//         TWIM1,
//     }, Peripherals,
// };

pub const SEG: [u8; 16] = [
    0x3F, 0x06, 0x5B, 0x4F, // 0~3
    0x66, 0x6D, 0x7D, 0x07, // 4~7
    0x7F, 0x6F, 0x77, 0x7C, // 8~B
    0x39, 0x5E, 0x79, 0x71  // C~F
];

pub const CMD_ADDR: u8 = 0x24;
pub const DISPLAY_ADDR:[u8; 4] = [
    0x34,
    0x35,
    0x36,
    0x37
];

// pub struct Tm1650 {
//     pub intensity: u8,
//     pub cmd: [u8; 1],
//     pub dbuf: [u8; 4],
// }

// impl Default for Tm1650 {
//     fn default() -> Tm1650 {
//         Tm1650 {
//             intensity: 3,
//             cmd: [49],
//             dbuf: [1, 1, 0, 2],
//         }
//     }
// }

// impl Tm1650 {
//     pub fn test(&mut self) -> () {
//         // let board = microbit::Board::take().unwrap();
//         let board = microbit::Board::new(
//             pac::Peripherals::take().unwrap(),
//             pac::CorePeripherals::take().unwrap(),
//         );
//         let mut i2c = twim::Twim::new(
//             board.TWIM0,
//             board.i2c_external.into(),
//             FREQUENCY_A::K100,
//         );
//         // let i2c = twim::Twim::<TWIM1>::new(
//         //     board.TWIM1,
//         //     board.i2c_external.into(),
//         //     FREQUENCY_A::K100,
//         // );
//         i2c.write(CMD_ADDR, &self.cmd);
//         for i in 0..4 {
//             self.cmd[0] = SEG[self.dbuf[3-i] as usize];
//             // rprintln!("{:?}",seg4);
//             i2c.write(DISPLAY_ADDR[i],
//                 &self.cmd);
//         }
//     }
// }

// pub fn tm1650_cmd  (
//     cmd: &mut Tm1650,
// ) -> !
// {
//     i2c = twim::Twim::new(

//     )
// }