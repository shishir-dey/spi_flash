use crate::error::*;
use crate::interface::{CsPin, Delay, SpiInterface};
use crate::types::*;

pub const PAGE_SIZE: u32 = 256;
pub const SECTOR_SIZE: u32 = 4096;
pub const BLOCK_SIZE: u32 = 65536;

#[allow(dead_code)]
pub struct SpiFlash<SPI: SpiInterface, CS: CsPin, Timer: Delay> {
    pub spi: SPI,
    pub cs: CS,
    pub timer: Timer,
    pub manufactor: Manufactor,
    pub size: Size,
    pub initialised: bool,
    pub memory_type: u8,
    pub lock: bool,
    pub reserved: u8,
    pub pin: u32,
    pub page_count: u32,
    pub sector_count: u32,
    pub block_count: u32,
}

impl<SPI: SpiInterface, CS: CsPin, Timer: Delay> SpiFlash<SPI, CS, Timer> {
    pub fn new(spi: SPI, cs: CS, timer: Timer) -> Self {
        Self {
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
        }
    }

    pub fn lock(&mut self) -> Result<(), SpiFlashError<SPI::SpiError, CS::IoError>> {
        while self.lock {
            self.timer.delay_us(1000);
        }
        self.lock = true;
        Ok(())
    }

    pub fn unlock(&mut self) -> Result<(), SpiFlashError<SPI::SpiError, CS::IoError>> {
        self.lock = false;
        Ok(())
    }

    pub fn cs_drive(
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

    pub fn write_enable(&mut self) -> Result<(), SpiFlashError<SPI::SpiError, CS::IoError>> {
        self.cs_drive(true);
        self.spi.write(&[Command::WriteEnable as u8]);
        self.cs_drive(false);
        Ok(())
    }

    pub fn write_disable(&mut self) -> Result<(), SpiFlashError<SPI::SpiError, CS::IoError>> {
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

    pub fn write_reg_1(
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

    pub fn write_reg_2(
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

    pub fn write_reg_3(
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

    pub fn wait_for_writing(
        &mut self,
        timeout: u32,
    ) -> Result<(), SpiFlashError<SPI::SpiError, CS::IoError>> {
        let rx = self.read_reg_1()?;
        while rx & Status1::Busy as u8 != 0 {
            self.timer.delay_us(timeout);
        }
        Ok(())
    }

    pub fn find_chip(&mut self) -> Result<(), SpiFlashError<SPI::SpiError, CS::IoError>> {
        self.cs_drive(true);
        self.spi
            .write(&[Command::JEDECID as u8, DUMMY_BYTE, DUMMY_BYTE, DUMMY_BYTE]);
        let mut rx = [0u8; 4];
        self.spi.read(&mut rx);
        self.cs_drive(false);
        match rx[1] {
            0xEF => self.manufactor = Manufactor::Winbond,
            0x9D => self.manufactor = Manufactor::Issi,
            0x20 => self.manufactor = Manufactor::Micron,
            0xC8 => self.manufactor = Manufactor::GigaDevice,
            0xC2 => self.manufactor = Manufactor::Macronix,
            0x01 => self.manufactor = Manufactor::Spansion,
            0x37 => self.manufactor = Manufactor::Amic,
            0xBF => self.manufactor = Manufactor::Sst,
            0xAD => self.manufactor = Manufactor::Hyundai,
            0x1F => self.manufactor = Manufactor::Atmel,
            0xA1 => self.manufactor = Manufactor::Fudan,
            0x8C => self.manufactor = Manufactor::Esmt,
            0x89 => self.manufactor = Manufactor::Intel,
            0x62 => self.manufactor = Manufactor::Sanyo,
            0x04 => self.manufactor = Manufactor::Fujitsu,
            0x1C => self.manufactor = Manufactor::Eon,
            0x85 => self.manufactor = Manufactor::Puya,
            _ => self.manufactor = Manufactor::Error,
        }
        self.memory_type = rx[2];
        match rx[3] {
            0x11 => self.size = Size::Mbit1,
            0x12 => self.size = Size::Mbit2,
            0x13 => self.size = Size::Mbit4,
            0x14 => self.size = Size::Mbit8,
            0x15 => self.size = Size::Mbit16,
            0x16 => self.size = Size::Mbit32,
            0x17 => self.size = Size::Mbit64,
            0x18 => self.size = Size::Mbit128,
            0x19 => self.size = Size::Mbit256,
            0x20 => self.size = Size::Mbit512,
            _ => self.size = Size::Error,
        }
        self.block_count = self.size as u32 * 16;
        self.sector_count = self.block_count * 16;
        self.page_count = self.sector_count * 16;
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
}
