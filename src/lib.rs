#![no_std]

/// Configuration
pub mod configuration;
/// The HC-12 device
pub mod device;
/// Modes
pub mod modes;

pub(crate) mod sealed {
    pub trait Sealed {}
}

pub use device::*;
pub use modes::*;
