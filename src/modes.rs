use core::fmt::Debug;

use crate::{
    baudrates::{ValidBaudrate, B9600},
    sealed::Sealed,
};
use defmt::Format;

pub trait ValidMode: Sealed + Copy + Debug + Format {}

/// A valid transparent mode for the HC-12
pub trait ValidTransparentMode: ValidMode {
    const CODE: &'static str;
}

/// A valid baudrate for a transparent mode
pub trait ModeValidWithBaudrate<B: ValidBaudrate>: ValidTransparentMode {
    fn from_baudrate(baudrate: B) -> Self;
}

/// AT Mode: Used for programming the HC-12. Because AT requires B9600 while programming, it stores the actual
/// programmed baudrate to discriminate between valid modes, elsewhere
#[derive(Copy, Clone, Debug, Format)]
pub struct AT<B: Copy + Clone + Debug + Format, M: ValidTransparentMode> {
    /// The programmed baudrate of the device. Distinct from the underlying UART baudrate!
    programmed_baudrate: B,
    /// The programmed transparent mode of the device
    programmed_mode: M,
}
impl<B: Copy + Clone + Debug + Format, M: ValidTransparentMode> Sealed for AT<B, M> {}
impl<B: Copy + Clone + Debug + Format, M: ValidTransparentMode> ValidMode for AT<B, M> {}
impl<B: Copy + Clone + Debug + Format, M: ValidTransparentMode> AT<B, M> {
    /// Create a new AT mode
    pub(crate) fn new(programmed_mode: M, programmed_baudrate: B) -> Self {
        AT {
            programmed_baudrate,
            programmed_mode,
        }
    }

    /// Get the programmed mode of the device
    pub(crate) fn programmed_mode(&self) -> M {
        self.programmed_mode
    }

    /// Get the programmed baudrate of the device
    pub(crate) fn programmed_baudrate(&self) -> B {
        self.programmed_baudrate
    }
}

/// FU3 Mode: The normal high-speed transparent mode of the HC-12
#[derive(Copy, Clone, Debug, Format)]
pub struct FU3<B: ValidBaudrate> {
    baudrate: B,
}
impl<B: ValidBaudrate> Sealed for FU3<B> {}
impl<B: ValidBaudrate> ValidMode for FU3<B> {}
impl<B: ValidBaudrate> ValidTransparentMode for FU3<B> {
    const CODE: &'static str = "FU3";
}
/// FU3 is a valid transparent mode with all baudrates
impl<B: ValidBaudrate> ModeValidWithBaudrate<B> for FU3<B> {
    fn from_baudrate(baudrate: B) -> Self {
        FU3 { baudrate }
    }
}
impl<B: ValidBaudrate> FU3<B> {
    /// Create a new FU3 mode
    pub(crate) fn new(baudrate: B) -> Self {
        FU3 { baudrate }
    }

    /// Get the baudrate of the mode
    pub fn baudrate(&self) -> B {
        self.baudrate
    }
}

impl Default for FU3<B9600> {
    fn default() -> Self {
        FU3 { baudrate: B9600 }
    }
}

/// An unknown transparent mode. This can't be built into, but is useful for building the device from scratch without a known mode
#[derive(Copy, Clone, Debug, Format)]
pub struct UnkownTransparentMode;
impl Sealed for UnkownTransparentMode {}
impl ValidMode for UnkownTransparentMode {}
impl ValidTransparentMode for UnkownTransparentMode {
    const CODE: &'static str = "NEVER_USE";
}

#[cfg(test)]
mod test {
    use crate::baudrates::{B115200, B9600};

    use super::*;
}
