use crate::error::*;
use crate::interface::{CsPin, Delay, SpiInterface};
use crate::misc::*;
use crate::types::*;

pub const PAGE_SIZE: u32 = 256;
pub const SECTOR_SIZE: u32 = 4096;
pub const BLOCK_SIZE: u32 = 65536;

#[allow(dead_code)]
pub struct SpiFlash<SPI: SpiInterface, CS: CsPin, Timer: Delay> {
    spi: SPI,
    cs: CS,
    timer: Timer,
    manufactor: Manufactor,
    size: Size,
    initialised: bool,
    memory_type: u8,
    lock: bool,
    reserved: u8,
    pin: u32,
    page_count: u32,
    sector_count: u32,
    block_count: u32,
}

impl<SPI: SpiInterface, CS: CsPin, Timer: Delay> SpiFlash<SPI, CS, Timer> {
    pub fn new(
        spi: SPI,
        cs: CS,
        timer: Timer,
    ) -> Result<Self, SpiFlashError<SPI::SpiError, CS::IoError>> {
        let mut dev = Self {
            spi,
            cs,
            timer,
            manufactor: Manufactor::Error,
            size: Size::Error,
            initialised: false,
            memory_type: 0,
            lock: false,
            reserved: 0,
            pin: 0,
            page_count: 0,
            sector_count: 0,
            block_count: 0,
        };

        dev.cs_drive(true);
        dev.spi
            .write(&[Command::JEDECID as u8, DUMMY_BYTE, DUMMY_BYTE, DUMMY_BYTE]);
        let mut rx = [0u8; 4];
        dev.spi.read(&mut rx);
        dev.cs_drive(false);
        match rx[1] {
            0xEF => dev.manufactor = Manufactor::Winbond,
            0x9D => dev.manufactor = Manufactor::Issi,
            0x20 => dev.manufactor = Manufactor::Micron,
            0xC8 => dev.manufactor = Manufactor::GigaDevice,
            0xC2 => dev.manufactor = Manufactor::Macronix,
            0x01 => dev.manufactor = Manufactor::Spansion,
            0x37 => dev.manufactor = Manufactor::Amic,
            0xBF => dev.manufactor = Manufactor::Sst,
            0xAD => dev.manufactor = Manufactor::Hyundai,
            0x1F => dev.manufactor = Manufactor::Atmel,
            0xA1 => dev.manufactor = Manufactor::Fudan,
            0x8C => dev.manufactor = Manufactor::Esmt,
            0x89 => dev.manufactor = Manufactor::Intel,
            0x62 => dev.manufactor = Manufactor::Sanyo,
            0x04 => dev.manufactor = Manufactor::Fujitsu,
            0x1C => dev.manufactor = Manufactor::Eon,
            0x85 => dev.manufactor = Manufactor::Puya,
            _ => dev.manufactor = Manufactor::Error,
        }
        dev.memory_type = rx[2];
        match rx[3] {
            0x11 => dev.size = Size::Mbit1,
            0x12 => dev.size = Size::Mbit2,
            0x13 => dev.size = Size::Mbit4,
            0x14 => dev.size = Size::Mbit8,
            0x15 => dev.size = Size::Mbit16,
            0x16 => dev.size = Size::Mbit32,
            0x17 => dev.size = Size::Mbit64,
            0x18 => dev.size = Size::Mbit128,
            0x19 => dev.size = Size::Mbit256,
            0x20 => dev.size = Size::Mbit512,
            _ => dev.size = Size::Error,
        }
        dev.block_count = dev.size as u32 * 16;
        dev.sector_count = dev.block_count * 16;
        dev.page_count = dev.sector_count * 16;

        Ok(dev)
    }

    fn lock(&mut self) -> Result<(), SpiFlashError<SPI::SpiError, CS::IoError>> {
        while self.lock {
            self.timer.delay_us(1000);
        }
        self.lock = true;
        Ok(())
    }

    fn unlock(&mut self) -> Result<(), SpiFlashError<SPI::SpiError, CS::IoError>> {
        self.lock = false;
        Ok(())
    }

    fn cs_drive(&mut self, state: bool) -> Result<(), SpiFlashError<SPI::SpiError, CS::IoError>> {
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

    fn write_enable(&mut self) -> Result<(), SpiFlashError<SPI::SpiError, CS::IoError>> {
        self.cs_drive(true);
        self.spi.write(&[Command::WriteEnable as u8]);
        self.cs_drive(false);
        Ok(())
    }

    fn write_disable(&mut self) -> Result<(), SpiFlashError<SPI::SpiError, CS::IoError>> {
        self.cs_drive(true);
        self.spi.write(&[Command::WriteDisable as u8]);
        self.cs_drive(false);
        Ok(())
    }

    fn read_reg_1(&mut self) -> Result<u8, SpiFlashError<SPI::SpiError, CS::IoError>> {
        self.cs_drive(true);
        self.spi.write(&[Command::ReadStatus1 as u8, DUMMY_BYTE]);
        let mut rx = [0u8; 2];
        self.spi.read(&mut rx);
        self.cs_drive(false);
        Ok(rx[1])
    }

    fn read_reg_2(&mut self) -> Result<u8, SpiFlashError<SPI::SpiError, CS::IoError>> {
        self.cs_drive(true);
        self.spi.write(&[Command::ReadStatus2 as u8, DUMMY_BYTE]);
        let mut rx = [0u8; 2];
        self.spi.read(&mut rx);
        self.cs_drive(false);
        Ok(rx[1])
    }

    fn read_reg_3(&mut self) -> Result<u8, SpiFlashError<SPI::SpiError, CS::IoError>> {
        self.cs_drive(true);
        self.spi.write(&[Command::ReadStatus3 as u8, DUMMY_BYTE]);
        let mut rx = [0u8; 2];
        self.spi.read(&mut rx);
        self.cs_drive(false);
        Ok(rx[1])
    }

    fn write_reg_1(&mut self, data: u8) -> Result<(), SpiFlashError<SPI::SpiError, CS::IoError>> {
        self.write_enable();
        self.cs_drive(true);
        self.spi.write(&[Command::WriteStatus1 as u8, data]);
        self.cs_drive(false);
        self.write_disable();
        Ok(())
    }

    fn write_reg_2(&mut self, data: u8) -> Result<(), SpiFlashError<SPI::SpiError, CS::IoError>> {
        self.write_enable();
        self.cs_drive(true);
        self.spi.write(&[Command::WriteStatus2 as u8, data]);
        self.cs_drive(false);
        self.write_disable();
        Ok(())
    }

    fn write_reg_3(&mut self, data: u8) -> Result<(), SpiFlashError<SPI::SpiError, CS::IoError>> {
        self.write_enable();
        self.cs_drive(true);
        self.spi.write(&[Command::WriteStatus3 as u8, data]);
        self.cs_drive(false);
        self.write_disable();
        Ok(())
    }

    fn wait_for_writing(
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

    fn write(
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
