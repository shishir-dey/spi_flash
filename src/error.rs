use core::fmt::Debug;

#[derive(Debug)]
pub enum SpiFlashError<SpiErr, IoErr> {
    Spi(SpiErr),
    Io(IoErr),
    Protocol,
}