use super::*;

impl<SPI: SpiInterface, CS: CsPin, Timer: Delay> SpiFlash<SPI, CS, Timer> {
    pub(crate) fn lock(&mut self) -> Result<(), SpiFlashError<SPI::SpiError, CS::IoError>> {
        while self.lock {
            self.timer.delay_us(1000);
        }
        self.lock = true;
        Ok(())
    }

    pub(crate) fn unlock(&mut self) -> Result<(), SpiFlashError<SPI::SpiError, CS::IoError>> {
        self.lock = false;
        Ok(())
    }

    pub(crate) fn cs_drive(
        &mut self,
        state: bool,
    ) -> Result<(), SpiFlashError<SPI::SpiError, CS::IoError>> {
        match state {
            true => {
                self.cs.set_low();
            }
            false => {
                self.cs.set_high();
            }
        }
        Ok(())
    }

    pub(crate) fn write_enable(&mut self) -> Result<(), SpiFlashError<SPI::SpiError, CS::IoError>> {
        self.cs_drive(true);
        self.spi.write(&[Command::WriteEnable as u8]);
        self.cs_drive(false);
        Ok(())
    }

    pub(crate) fn write_disable(
        &mut self,
    ) -> Result<(), SpiFlashError<SPI::SpiError, CS::IoError>> {
        self.cs_drive(true);
        self.spi.write(&[Command::WriteDisable as u8]);
        self.cs_drive(false);
        Ok(())
    }

    pub(crate) fn read_reg_1(&mut self) -> Result<u8, SpiFlashError<SPI::SpiError, CS::IoError>> {
        self.cs_drive(true);
        self.spi.write(&[Command::ReadStatus1 as u8, DUMMY_BYTE]);
        let mut rx = [0u8; 2];
        self.spi.read(&mut rx);
        self.cs_drive(false);
        Ok(rx[1])
    }

    pub(crate) fn read_reg_2(&mut self) -> Result<u8, SpiFlashError<SPI::SpiError, CS::IoError>> {
        self.cs_drive(true);
        self.spi.write(&[Command::ReadStatus2 as u8, DUMMY_BYTE]);
        let mut rx = [0u8; 2];
        self.spi.read(&mut rx);
        self.cs_drive(false);
        Ok(rx[1])
    }

    pub(crate) fn read_reg_3(&mut self) -> Result<u8, SpiFlashError<SPI::SpiError, CS::IoError>> {
        self.cs_drive(true);
        self.spi.write(&[Command::ReadStatus3 as u8, DUMMY_BYTE]);
        let mut rx = [0u8; 2];
        self.spi.read(&mut rx);
        self.cs_drive(false);
        Ok(rx[1])
    }

    pub(crate) fn write_reg_1(
        &mut self,
        data: u8,
    ) -> Result<(), SpiFlashError<SPI::SpiError, CS::IoError>> {
        self.write_enable();
        self.cs_drive(true);
        self.spi.write(&[Command::WriteStatus1 as u8, data]);
        self.cs_drive(false);
        self.write_disable();
        Ok(())
    }

    pub(crate) fn write_reg_2(
        &mut self,
        data: u8,
    ) -> Result<(), SpiFlashError<SPI::SpiError, CS::IoError>> {
        self.write_enable();
        self.cs_drive(true);
        self.spi.write(&[Command::WriteStatus2 as u8, data]);
        self.cs_drive(false);
        self.write_disable();
        Ok(())
    }

    pub(crate) fn write_reg_3(
        &mut self,
        data: u8,
    ) -> Result<(), SpiFlashError<SPI::SpiError, CS::IoError>> {
        self.write_enable();
        self.cs_drive(true);
        self.spi.write(&[Command::WriteStatus3 as u8, data]);
        self.cs_drive(false);
        self.write_disable();
        Ok(())
    }

    pub(crate) fn wait_for_writing(
        &mut self,
        timeout: u32,
    ) -> Result<(), SpiFlashError<SPI::SpiError, CS::IoError>> {
        loop {
            let rx = self.read_reg_1()?;
            if rx & Status1::Busy as u8 == 0 {
                break;
            }
            self.timer.delay_us(timeout);
        }
        Ok(())
    }

    pub(crate) fn write(
        &mut self,
        page_number: u32,
        data: &[u8],
        size: u32,
        offset: u32,
    ) -> Result<(), SpiFlashError<SPI::SpiError, CS::IoError>> {
        if page_number >= self.page_count {
            return Err(SpiFlashError::Protocol);
        }

        if offset >= PAGE_SIZE {
            return Err(SpiFlashError::Protocol);
        }

        let maximum = PAGE_SIZE - offset;
        let size = if size > maximum { maximum } else { size } as usize;

        let address = page_to_address(page_number) + offset;

        self.lock()?;
        self.write_enable()?;
        self.cs_drive(true)?;

        if self.block_count >= 512 {
            self.spi
                .write(&[
                    Command::PageProg4Add as u8,
                    ((address & 0xFF000000) >> 24) as u8,
                    ((address & 0x00FF0000) >> 16) as u8,
                    ((address & 0x0000FF00) >> 8) as u8,
                    (address & 0x000000FF) as u8,
                ])
                .map_err(|e| {
                    let _ = self.cs_drive(false);
                    SpiFlashError::Spi(e)
                })?;
        } else {
            self.spi
                .write(&[
                    Command::PageProg3Add as u8,
                    ((address & 0x00FF0000) >> 16) as u8,
                    ((address & 0x0000FF00) >> 8) as u8,
                    (address & 0x000000FF) as u8,
                ])
                .map_err(|e| {
                    let _ = self.cs_drive(false);
                    SpiFlashError::Spi(e)
                })?;
        }

        self.spi.write(&data[..size]).map_err(|e| {
            let _ = self.cs_drive(false);
            SpiFlashError::Spi(e)
        })?;

        self.cs_drive(false)?;
        self.wait_for_writing(100)?;
        self.write_disable()?;
        self.unlock()?;

        Ok(())
    }
}
