use super::*;

impl<SPI: SpiInterface, CS: CsPin, Timer: Delay> SpiFlash<SPI, CS, Timer> {}
