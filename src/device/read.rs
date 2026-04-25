use super::*;

impl<SPI: SpiInterface, CS: CsPin, Timer: Delay> SpiFlash<SPI, CS, Timer> {
    pub fn read_page(
        &mut self,
        page_number: u32,
        data: &mut [u8],
        size: u32,
        offset: u32,
    ) -> Result<(), SpiFlashError<SPI::SpiError, CS::IoError>> {
        self.lock()?;

        if offset >= PAGE_SIZE {
            self.unlock()?;
            return Err(SpiFlashError::Protocol);
        }

        let address = page_to_address(page_number) + offset;
        let size = size.min(PAGE_SIZE - offset);

        self.read(address, data, size).map_err(|e| {
            let _ = self.unlock();
            e
        })?;

        self.unlock()?;
        Ok(())
    }

    pub fn read_sector(
        &mut self,
        sector_number: u32,
        data: &mut [u8],
        size: u32,
        offset: u32,
    ) -> Result<(), SpiFlashError<SPI::SpiError, CS::IoError>> {
        self.lock()?;

        if offset >= SECTOR_SIZE {
            self.unlock()?;
            return Err(SpiFlashError::Protocol);
        }

        let address = sector_to_address(sector_number) + offset;
        let size = size.min(SECTOR_SIZE - offset);

        self.read(address, data, size).map_err(|e| {
            let _ = self.unlock();
            e
        })?;

        self.unlock()?;
        Ok(())
    }

    pub fn read_block(
        &mut self,
        block_number: u32,
        data: &mut [u8],
        size: u32,
        offset: u32,
    ) -> Result<(), SpiFlashError<SPI::SpiError, CS::IoError>> {
        self.lock()?;

        if offset >= BLOCK_SIZE {
            self.unlock()?;
            return Err(SpiFlashError::Protocol);
        }

        let address = block_to_address(block_number) + offset;
        let size = size.min(BLOCK_SIZE - offset);

        self.read(address, data, size).map_err(|e| {
            let _ = self.unlock();
            e
        })?;

        self.unlock()?;
        Ok(())
    }
}
