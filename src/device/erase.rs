use super::*;

impl<SPI: SpiInterface, CS: CsPin, Timer: Delay> SpiFlash<SPI, CS, Timer> {
    pub fn erase_chip(&mut self) -> Result<(), SpiFlashError<SPI::SpiError, CS::IoError>> {
        self.lock();
        self.write_enable();
        self.cs_drive(true);
        self.spi.write(&[Command::ChipErase1 as u8]);
        self.cs_drive(false);
        self.wait_for_writing(100);
        self.write_disable();
        self.unlock();
        Ok(())
    }

    pub fn erase_sector(
        &mut self,
        sector_number: u32,
    ) -> Result<(), SpiFlashError<SPI::SpiError, CS::IoError>> {
        self.lock();
        self.write_enable();
        self.cs_drive(true);
        self.spi.write(&[
            Command::SectorErase4Add as u8,
            ((sector_number * SECTOR_SIZE) >> 16) as u8,
            ((sector_number * SECTOR_SIZE) >> 8) as u8,
            (sector_number * SECTOR_SIZE) as u8,
        ]);
        self.cs_drive(false);
        self.wait_for_writing(100);
        self.write_disable();
        self.unlock();
        Ok(())
    }

    pub fn erase_block(
        &mut self,
        block_number: u32,
    ) -> Result<(), SpiFlashError<SPI::SpiError, CS::IoError>> {
        self.lock();
        self.write_enable();
        self.cs_drive(true);
        self.spi.write(&[
            Command::BlockErase4Add as u8,
            ((block_number * BLOCK_SIZE) >> 16) as u8,
            ((block_number * BLOCK_SIZE) >> 8) as u8,
            (block_number * BLOCK_SIZE) as u8,
        ]);
        self.cs_drive(false);
        self.wait_for_writing(100);
        self.write_disable();
        self.unlock();
        Ok(())
    }
}
