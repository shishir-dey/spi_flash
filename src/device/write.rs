use super::*;

impl<SPI: SpiInterface, CS: CsPin, Timer: Delay> SpiFlash<SPI, CS, Timer> {
    pub fn write_address(
        &mut self,
        address: u32,
        data: &[u8],
        size: u32,
    ) -> Result<(), SpiFlashError<SPI::SpiError, CS::IoError>> {
        self.lock()?;

        let mut add = address;
        let mut remaining = size;
        let mut index: u32 = 0;

        loop {
            let page = address_to_page(add);
            let offset = add % PAGE_SIZE;
            let maximum = PAGE_SIZE - offset;
            let length = if remaining <= maximum {
                remaining
            } else {
                maximum
            };

            self.write(
                page,
                &data[index as usize..(index + length) as usize],
                length,
                offset,
            )
            .map_err(|e| {
                let _ = self.unlock();
                e
            })?;

            add += length;
            index += length;
            remaining -= length;

            if remaining == 0 {
                break;
            }
        }

        self.unlock()?;
        Ok(())
    }

    pub fn write_page(
        &mut self,
        page_number: u32,
        data: &[u8],
        size: u32,
        offset: u32,
    ) -> Result<(), SpiFlashError<SPI::SpiError, CS::IoError>> {
        self.lock()?;
        self.write(page_number, data, size, offset)?;
        self.unlock()?;
        Ok(())
    }

    pub fn write_sector(
        &mut self,
        sector_number: u32,
        data: &[u8],
        size: u32,
        offset: u32,
    ) -> Result<(), SpiFlashError<SPI::SpiError, CS::IoError>> {
        self.lock()?;

        if offset >= SECTOR_SIZE {
            self.unlock()?;
            return Err(SpiFlashError::Protocol);
        }

        let size = size.min(SECTOR_SIZE - offset);
        let mut bytes_written: u32 = 0;
        let mut remaining_bytes = size;
        let mut page_number = sector_number * (SECTOR_SIZE / PAGE_SIZE) + offset / PAGE_SIZE;
        let mut page_offset = offset % PAGE_SIZE;
        let page_limit = (sector_number + 1) * (SECTOR_SIZE / PAGE_SIZE);

        while remaining_bytes > 0 && page_number < page_limit {
            let bytes_to_write = remaining_bytes.min(PAGE_SIZE - page_offset);

            self.write(
                page_number,
                &data[bytes_written as usize..(bytes_written + bytes_to_write) as usize],
                bytes_to_write,
                page_offset,
            )
            .map_err(|e| {
                let _ = self.unlock();
                e
            })?;

            bytes_written += bytes_to_write;
            remaining_bytes -= bytes_to_write;
            page_number += 1;
            page_offset = 0;
        }

        self.unlock()?;
        Ok(())
    }

    pub fn write_block(
        &mut self,
        block_number: u32,
        data: &[u8],
        size: u32,
        offset: u32,
    ) -> Result<(), SpiFlashError<SPI::SpiError, CS::IoError>> {
        self.lock()?;

        if offset >= BLOCK_SIZE {
            self.unlock()?;
            return Err(SpiFlashError::Protocol);
        }

        let size = size.min(BLOCK_SIZE - offset);
        let mut bytes_written: u32 = 0;
        let mut remaining_bytes = size;
        let mut page_number = block_number * (BLOCK_SIZE / PAGE_SIZE) + offset / PAGE_SIZE;
        let mut page_offset = offset % PAGE_SIZE;
        let page_limit = (block_number + 1) * (BLOCK_SIZE / PAGE_SIZE);

        while remaining_bytes > 0 && page_number < page_limit {
            let bytes_to_write = remaining_bytes.min(PAGE_SIZE - page_offset);

            self.write(
                page_number,
                &data[bytes_written as usize..(bytes_written + bytes_to_write) as usize],
                bytes_to_write,
                page_offset,
            )
            .map_err(|e| {
                let _ = self.unlock();
                e
            })?;

            bytes_written += bytes_to_write;
            remaining_bytes -= bytes_to_write;
            page_number += 1;
            page_offset = 0;
        }

        self.unlock()?;
        Ok(())
    }
}
