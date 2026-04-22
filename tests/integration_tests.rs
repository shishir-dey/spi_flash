#[cfg(any(test, feature = "std"))]
use spi_flash::device::SpiFlash;
use spi_flash::mock::{MockCs, MockDelay, MockSpi};
use spi_flash::types::{Manufactor, Size};

#[test]
fn test_always_passes() {
    assert!(true);
}
