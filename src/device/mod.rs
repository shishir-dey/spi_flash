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

    pub fn manufactor(&self) -> Manufactor {
        self.manufactor
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn memory_type(&self) -> u8 {
        self.memory_type
    }

    pub fn page_count(&self) -> u32 {
        self.page_count
    }

    pub fn sector_count(&self) -> u32 {
        self.sector_count
    }

    pub fn block_count(&self) -> u32 {
        self.block_count
    }
}

mod erase;
mod internal;
mod read;
mod write;
