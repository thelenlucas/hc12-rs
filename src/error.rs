use core::fmt::Debug;

use heapless::String;

use crate::paramaters::BadChannel;

/// An error in creating a device, for some internal or an underlying issue
#[derive(Debug)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum Error<D: Debug> {
    /// Underlying device error
    DeviceError(D),
    /// An invalid channel was selected
    BadChannel(u8),
    /// No response was recieved
    NoResponse,
    /// A non-ok response was recieved
    NoOK(String<16>),
}

impl<D: embedded_io::Error> From<D> for Error<D> {
    fn from(value: D) -> Self {
        Error::DeviceError(value)
    }
}

impl<D: core::fmt::Debug> From<BadChannel> for Error<D> {
    fn from(value: BadChannel) -> Self {
        Self::BadChannel(value.into())
    }
}
