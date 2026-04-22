#![cfg(any(test, feature = "std"))]

extern crate std;

use crate::interface::{CsPin, Delay, SpiInterface};
use std::vec::Vec;

#[derive(Debug, Clone, Copy)]
pub enum MockError {
    Generic,
}

pub struct MockCs {
    pub state_low: bool,
}

impl MockCs {
    pub fn new() -> Self {
        Self { state_low: false }
    }
}

impl CsPin for MockCs {
    type IoError = ();

    fn set_low(&mut self) -> Result<(), Self::IoError> {
        self.state_low = true;
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::IoError> {
        self.state_low = false;
        Ok(())
    }
}

pub struct MockDelay;

impl Delay for MockDelay {
    fn delay_ms(&mut self, _ms: u32) {}
}
pub struct MockSpi {
    last_tx: Vec<u8>,
    pub write_enabled: bool,
    pub jedec_id: [u8; 3],
    pub status1: u8,
    pub status2: u8,
    pub status3: u8,
}

impl MockSpi {
    pub fn new() -> Self {
        Self {
            last_tx: Vec::new(),
            write_enabled: false,
            jedec_id: [0xEF, 0x40, 0x18],
            status1: 0,
            status2: 0,
            status3: 0,
        }
    }
}

impl SpiInterface for MockSpi {
    type SpiError = MockError;

    fn write(&mut self, tx: &[u8]) -> Result<(), Self::SpiError> {
        self.last_tx.clear();
        self.last_tx.extend_from_slice(tx);

        match tx[0] {
            0x06 => self.write_enabled = true,
            0x04 => self.write_enabled = false,
            0x01 => {
                if tx.len() > 1 {
                    self.status1 = tx[1];
                }
            }
            0x31 => {
                if tx.len() > 1 {
                    self.status2 = tx[1];
                }
            }
            0x11 => {
                if tx.len() > 1 {
                    self.status3 = tx[1];
                }
            }
            _ => {}
        }

        Ok(())
    }

    fn read(&mut self, rx: &mut [u8]) -> Result<(), Self::SpiError> {
        if self.last_tx.is_empty() {
            return Ok(());
        }

        match self.last_tx[0] {
            0x9F => {
                if rx.len() >= 4 {
                    rx[0] = 0;
                    rx[1] = self.jedec_id[0];
                    rx[2] = self.jedec_id[1];
                    rx[3] = self.jedec_id[2];
                }
            }
            0x05 => {
                if rx.len() >= 2 {
                    rx[1] = self.status1;
                }
            }
            0x35 => {
                if rx.len() >= 2 {
                    rx[1] = self.status2;
                }
            }
            0x15 => {
                if rx.len() >= 2 {
                    rx[1] = self.status3;
                }
            }
            _ => {}
        }

        Ok(())
    }
}
