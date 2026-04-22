use core::fmt::Debug;

pub trait SpiInterface {
    type SpiError: Debug;

    fn write(&mut self, tx: &[u8]) -> Result<(), Self::SpiError>;
    fn read(&mut self, rx: &mut [u8]) -> Result<(), Self::SpiError>;
}

pub trait SpiTransfer: SpiInterface {
    fn transfer(&mut self, tx: &[u8], rx: &mut [u8]) -> Result<(), Self::SpiError>;
}

pub trait CsPin {
    type IoError;

    fn set_low(&mut self) -> Result<(), Self::IoError>;
    fn set_high(&mut self) -> Result<(), Self::IoError>;
}

pub trait Delay {
    fn delay_ms(&mut self, ms: u32);

    fn delay_us(&mut self, us: u32) {
        let ms = (us + 999) / 1000;
        self.delay_ms(ms);
    }
}
