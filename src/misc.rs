use crate::device::BLOCK_SIZE;
use crate::device::PAGE_SIZE;
use crate::device::SECTOR_SIZE;

pub fn page_to_sector(page_number: u32) -> u32 {
    (page_number * PAGE_SIZE) / SECTOR_SIZE
}

pub fn page_to_block(page_number: u32) -> u32 {
    (page_number * PAGE_SIZE) / BLOCK_SIZE
}

pub fn sector_to_block(sector_number: u32) -> u32 {
    (sector_number * SECTOR_SIZE) / BLOCK_SIZE
}

pub fn sector_to_page(sector_number: u32) -> u32 {
    (sector_number * SECTOR_SIZE) / PAGE_SIZE
}

pub fn block_to_page(block_number: u32) -> u32 {
    (block_number * BLOCK_SIZE) / PAGE_SIZE
}

pub fn page_to_address(page_number: u32) -> u32 {
    page_number * PAGE_SIZE
}

pub fn sector_to_address(sector_number: u32) -> u32 {
    sector_number * SECTOR_SIZE
}

pub fn block_to_address(block_number: u32) -> u32 {
    block_number * BLOCK_SIZE
}

pub fn address_to_page(address: u32) -> u32 {
    address / PAGE_SIZE
}

pub fn address_to_sector(address: u32) -> u32 {
    address / SECTOR_SIZE
}

pub fn address_to_block(address: u32) -> u32 {
    address / BLOCK_SIZE
}
