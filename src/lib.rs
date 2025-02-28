//! # `hc12-rs`
//!
//! A strongly-typed `#[no_std]` driver for the hc01 hc-12 radio module.
//!
//! This crate takes a few departures from a typical `embedded-hal` or `embedded-io` driver, due to some shortcomings with the way the way the two crates handle UART devices. The HC-12 is highly dependent on the speed of the underlying UART, as it has different speeds, and modes can be configurable for multiple speeds. Unfortunately, neither `embedded-hal` nor `embedded-io` provide a way to set the speed of the underlying UART, so this driver provides additional functionality to configure the device.
//!
//! ## Model
//! `hc12-rs` models the HC-12 as a collection of:
//! 1. The underlying UART `embedded-io` device, used to send and receive data
//! 2. An [OutputPin](https://docs.rs/embedded-hal/1.0.0/embedded_hal/digital/trait.OutputPin.html), used to move into and out of AT mode
//! 3. A [DelayNs](https://docs.rs/embedded-hal/latest/embedded_hal/delay/trait.DelayNs.html) device, used to provide a delay for the programming pin
//!
//! The device may be initialized without a programming pin or delay, but will do so only into one of the transparent modes, and will not be able to transition out of them.
//! Note that due to the OutputPin's traits, the transition between modes can fail. When setup fails, the underyling UART and programming resources are returned, so the user can handle the error.

// Only do no_std when test isn't enabled
#![cfg_attr(not(test), no_std)]

pub mod baudrates;
pub mod configuration;
pub mod device;
pub mod modes;
mod programming;

mod sealed {
    pub trait Sealed {}
}
