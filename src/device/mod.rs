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
    manufacturer: Manufacturer,
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
            manufacturer: Manufacturer::Error,
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
            0xEF => dev.manufacturer = Manufacturer::Winbond,
            0x9D => dev.manufacturer = Manufacturer::Issi,
            0x20 => dev.manufacturer = Manufacturer::Micron,
            0xC8 => dev.manufacturer = Manufacturer::GigaDevice,
            0xC2 => dev.manufacturer = Manufacturer::Macronix,
            0x01 => dev.manufacturer = Manufacturer::Spansion,
            0x37 => dev.manufacturer = Manufacturer::Amic,
            0xBF => dev.manufacturer = Manufacturer::Sst,
            0xAD => dev.manufacturer = Manufacturer::Hyundai,
            0x1F => dev.manufacturer = Manufacturer::Atmel,
            0xA1 => dev.manufacturer = Manufacturer::Fudan,
            0x8C => dev.manufacturer = Manufacturer::Esmt,
            0x89 => dev.manufacturer = Manufacturer::Intel,
            0x62 => dev.manufacturer = Manufacturer::Sanyo,
            0x04 => dev.manufacturer = Manufacturer::Fujitsu,
            0x1C => dev.manufacturer = Manufacturer::Eon,
            0x85 => dev.manufacturer = Manufacturer::Puya,
            _ => dev.manufacturer = Manufacturer::Error,
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

    pub fn manufacturer(&self) -> Manufacturer {
        self.manufacturer
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
