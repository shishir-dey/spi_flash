#![no_std]

mod device;
pub mod interface;
mod misc;
mod types;

pub use device::SpiFlash;
pub use misc::*;
pub use types::*;
