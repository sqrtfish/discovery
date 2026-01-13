use embedded_hal::i2c::{ErrorType, I2c, SevenBitAddress};
use embedded_hal::digital::{InputPin, OutputPin};

/// 软件实现的 I2C 总线
pub struct SoftI2C<SCL, SDA, DELAY> {
    scl: SCL,    // SCL 引脚
    sda: SDA,    // SDA 引脚
    delay: DELAY, // 延迟函数
}

impl<SCL, SDA, DELAY> SoftI2C<SCL, SDA, DELAY>
where
    SCL: OutputPin,
    SDA: OutputPin + InputPin,
    DELAY: FnMut(u32),
{
    /// 创建一个新的 SoftI2C 实例
    pub fn new(scl: SCL, sda: SDA, delay: DELAY) -> Self {
        SoftI2C { scl, sda, delay }
    }

    /// 发送起始条件
    fn start(&mut self) -> Result<(), ()> {
        self.sda.set_high().map_err(|_| ())?;
        (self.delay)(5);
        self.scl.set_high().map_err(|_| ())?;
        (self.delay)(5);
        self.sda.set_low().map_err(|_| ())?;
        (self.delay)(5);
        self.scl.set_low().map_err(|_| ())?;
        (self.delay)(5);
        Ok(())
    }

    /// 发送停止条件
    fn stop(&mut self) -> Result<(), ()> {
        self.sda.set_low().map_err(|_| ())?;
        (self.delay)(5);
        self.scl.set_high().map_err(|_| ())?;
        (self.delay)(5);
        self.sda.set_high().map_err(|_| ())?;
        (self.delay)(5);
        Ok(())
    }

    /// 发送一个字节
    fn write_byte(&mut self, byte: u8) -> Result<(), ()> {
        for i in 0..8 {
            if (byte << i) & 0x80 != 0 {
                self.sda.set_high().map_err(|_| ())?;
            } else {
                self.sda.set_low().map_err(|_| ())?;
            }
            (self.delay)(5);
            self.scl.set_high().map_err(|_| ())?;
            (self.delay)(5);
            self.scl.set_low().map_err(|_| ())?;
            (self.delay)(5);
        }
        // 读取 ACK
        self.sda.set_high().map_err(|_| ())?;
        (self.delay)(5);
        self.scl.set_high().map_err(|_| ())?;
        (self.delay)(5);
        let ack = self.sda.is_low().map_err(|_| ())?;
        self.scl.set_low().map_err(|_| ())?;
        (self.delay)(5);
        if ack {
            Ok(())
        } else {
            Err(())
        }
    }

    /// 读取一个字节
    fn read_byte(&mut self, ack: bool) -> Result<u8, ()> {
        let mut byte = 0;
        self.sda.set_high().map_err(|_| ())?;
        for i in 0..8 {
            (self.delay)(5);
            self.scl.set_high().map_err(|_| ())?;
            (self.delay)(5);
            if self.sda.is_high().map_err(|_| ())? {
                byte |= 1 << (7 - i);
            }
            self.scl.set_low().map_err(|_| ())?;
            (self.delay)(5);
        }
        // 发送 ACK/NACK
        if ack {
            self.sda.set_low().map_err(|_| ())?;
        } else {
            self.sda.set_high().map_err(|_| ())?;
        }
        (self.delay)(5);
        self.scl.set_high().map_err(|_| ())?;
        (self.delay)(5);
        self.scl.set_low().map_err(|_| ())?;
        (self.delay)(5);
        self.sda.set_high().map_err(|_| ())?;
        Ok(byte)
    }
}

impl<SCL, SDA, DELAY> ErrorType for SoftI2C<SCL, SDA, DELAY> {
    type Error = ();
}

impl<SCL, SDA, DELAY> I2c<SevenBitAddress> for SoftI2C<SCL, SDA, DELAY>
where
    SCL: OutputPin,
    SDA: OutputPin + InputPin,
    DELAY: FnMut(u32),
{
    fn read(&mut self, address: SevenBitAddress, buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.start()?;
        self.write_byte((address << 1) | 1)?;
        for i in 0..buffer.len() {
            buffer[i] = self.read_byte(i < buffer.len() - 1)?;
        }
        self.stop()?;
        Ok(())
    }

    fn write(&mut self, address: SevenBitAddress, bytes: &[u8]) -> Result<(), Self::Error> {
        self.start()?;
        self.write_byte(address << 1)?;
        for byte in bytes {
            self.write_byte(*byte)?;
        }
        self.stop()?;
        Ok(())
    }

    fn write_read(
        &mut self,
        address: SevenBitAddress,
        bytes: &[u8],
        buffer: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.write(address, bytes)?;
        self.read(address, buffer)?;
        Ok(())
    }
}