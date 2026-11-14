// PN532 over I2C without IRQ, polls the ready byte with a delay so a shared bus isn't starved

use embassy_time::Timer;
use embedded_hal_async::i2c::{I2c, Operation};
use esp_hal::gpio::Output;

const PN532_I2C_ADDRESS: u8 = 0x24;
const PN532_I2C_READY: u8 = 0x01;

pub struct Pn532I2cInterface<I2C> {
    i2c: I2C,
    reset: Output<'static>, // RSTPDN, open drain
}

impl<I2C: I2c> Pn532I2cInterface<I2C> {
    pub fn new(i2c: I2C, reset: Output<'static>) -> Self {
        Self { i2c, reset }
    }

    pub async fn hard_reset(&mut self) {
        self.reset.set_low();
        Timer::after_millis(50).await;
        self.reset.set_high();
        Timer::after_millis(150).await; // PN532 boot time
    }
}

impl<I2C: I2c> pn532::Interface for Pn532I2cInterface<I2C> {
    type Error = I2C::Error;

    async fn write(&mut self, frame: &mut [u8]) -> Result<(), Self::Error> {
        self.i2c.write(PN532_I2C_ADDRESS, frame).await
    }

    async fn wait_ready(&mut self) -> Result<(), Self::Error> {
        // PN532 may NACK while busy, so any error is just not ready (https://github.com/WMT-GmbH/pn532/issues/4)
        loop {
            let mut status = [0u8];
            if self.i2c.read(PN532_I2C_ADDRESS, &mut status).await.is_ok()
                && status[0] == PN532_I2C_READY
            {
                return Ok(());
            }
            Timer::after_millis(2).await;
        }
    }

    async fn read(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
        // skip ready byte
        self.i2c
            .transaction(
                PN532_I2C_ADDRESS,
                &mut [Operation::Read(&mut [0]), Operation::Read(buf)],
            )
            .await
    }

    async fn wake_up(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}
