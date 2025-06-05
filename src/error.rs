use heapless::String;

use crate::paramaters::BadChannel;

/// An error in creating a device, for some internal or an underlying issue
#[derive(Debug)]
pub enum Error<D: core::fmt::Debug, E: core::fmt::Debug> {
    /// Underlying device error
    SerialDevice(D),
    /// Underling pin error
    PinError(E),
    /// An invalid channel was selected
    BadChannel(u8),
    /// No response was recieved
    NoResponse,
    /// A non-ok response was recieved
    NoOk(String<16>),
}

impl<D: embedded_io::Error, E: core::fmt::Debug> From<D> for Error<D, E> {
    fn from(value: D) -> Self {
        Error::SerialDevice(value)
    }
}

impl<D: core::fmt::Debug, E: core::fmt::Debug> From<BadChannel> for Error<D, E> {
    fn from(value: BadChannel) -> Self {
        Self::BadChannel(value.into())
    }
}
