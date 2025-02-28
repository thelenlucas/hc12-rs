use embedded_hal::digital::ErrorType as DigitalErrorType;
use embedded_hal::{delay::DelayNs, digital::OutputPin};
use embedded_io::{ErrorType, Read, ReadReady, Write, WriteReady};

use crate::{
    configuration::Baudrate,
    modes::{ValidHC12Mode, ValidTransparentMode},
    sealed::Sealed,
};

/// A trait to mark that a pair of resources is valid for programming the HC-12
pub trait ValidProgrammingResources: Sealed {}

/// A pair of a pin and a delay, used for programming the HC-12
pub struct ProgrammingPair<P, D> {
    pub pin: P,
    pub delay: D,
}
impl<P, D> Sealed for ProgrammingPair<P, D>
where
    P: OutputPin,
    D: DelayNs,
{
}
impl<P, D> ValidProgrammingResources for ProgrammingPair<P, D>
where
    P: OutputPin,
    D: DelayNs,
{
}
impl<P, D> DigitalErrorType for ProgrammingPair<P, D>
where
    P: DigitalErrorType,
{
    type Error = P::Error;
}
// Re-impliment OutputPin on ProgrammingPair when the pin is an OutputPin
impl<P, D> OutputPin for ProgrammingPair<P, D>
where
    P: OutputPin,
{
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.pin.set_high()
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.pin.set_low()
    }
}
// Re-impliment DelayNs on ProgrammingPair when the delay is a DelayNs
impl<P, D> DelayNs for ProgrammingPair<P, D>
where
    D: DelayNs,
{
    fn delay_ns(&mut self, ns: u32) {
        self.delay.delay_ns(ns)
    }
}

/// HC-12 Device. Can be initialized either with a DelayNs item and a Pin, or without.
pub struct HC12<U, R, M, B> {
    pub(crate) uart: U,
    pub(crate) programming: R,
    mode: M,
    baud: B,
}

impl<U, R, M, B> Sealed for HC12<U, R, M, B> {}

impl<U, R, M: ValidHC12Mode, B: Baudrate> HC12<U, R, M, B> {
    pub(crate) fn new(uart: U, programming: R, mode: M, baud: B) -> Self {
        HC12 {
            uart,
            programming,
            mode,
            baud,
        }
    }

    pub(crate) fn into_inner(self) -> (U, R, M, B) {
        (self.uart, self.programming, self.mode, self.baud)
    }
}

// Errortype implimententation - we pull the error type from the UART
impl<U, R, M, B> ErrorType for HC12<U, R, M, B>
where
    U: ErrorType,
{
    type Error = U::Error;
}

// Read passthrough
impl<U, R, M, B> Read for HC12<U, R, M, B>
where
    U: Read,
    M: ValidTransparentMode,
{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.uart.read(buf)
    }
}

// ReadReady passthrough
impl<U, R, M, B> ReadReady for HC12<U, R, M, B>
where
    U: ReadReady,
    M: ValidTransparentMode,
{
    fn read_ready(&mut self) -> Result<bool, Self::Error> {
        self.uart.read_ready()
    }
}

// Write passthrough
impl<U, R, M, B> Write for HC12<U, R, M, B>
where
    U: Write,
    M: ValidTransparentMode,
{
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.uart.write(buf)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.uart.flush()
    }
}

// WriteReady passthrough
impl<U, R, M, B> WriteReady for HC12<U, R, M, B>
where
    U: WriteReady,
    M: ValidTransparentMode,
{
    fn write_ready(&mut self) -> Result<bool, Self::Error> {
        self.uart.write_ready()
    }
}
