use crate::{commands::Command, speeds::ValidSpeed};

/// A valid Mode for the HC12
pub trait ValidMode: Default + Command {}

/// A valid speed combination for a mode
pub trait ValidModeFor<Speed: ValidSpeed>: ValidMode {}

/// Moderate power saving mode, draws 3.6mA. Can be set to any speed
#[derive(Default)]
pub struct Fu1 {}
impl ValidMode for Fu1 {}
impl Command for Fu1 {
    fn command(&self) -> heapless::String<16> {
        "AT+FU1".try_into().unwrap()
    }
}

/// Extreme power saving mode, only supports 1200, 2400, and 4800 BPS
#[derive(Default)]
pub struct Fu2 {}
impl ValidMode for Fu2 {}
impl Command for Fu2 {
    fn command(&self) -> heapless::String<16> {
        "AT+FU2".try_into().unwrap()
    }
}
/// Standard full-speed mode, any speed supported
#[derive(Default)]
pub struct Fu3 {}
impl ValidMode for Fu3 {}
impl Command for Fu3 {
    fn command(&self) -> heapless::String<16> {
        "AT+FU3".try_into().unwrap()
    }
}

/// Maximum range mode, only supports 1200 BPS
#[derive(Default)]
pub struct Fu4 {}
impl ValidMode for Fu4 {}
impl Command for Fu4 {
    fn command(&self) -> heapless::String<16> {
        "AT+FU4".try_into().unwrap()
    }
}

impl<T: ValidSpeed> ValidModeFor<T> for Fu3 {}
