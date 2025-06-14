use crate::{
    commands::Command,
    speeds::{ValidSpeed, B1200, B2400, B4800},
};

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

impl<T: ValidSpeed> ValidModeFor<T> for Fu1 {}
impl<T: ValidSpeed> ValidModeFor<T> for Fu3 {}

impl ValidModeFor<B1200> for Fu2 {}
impl ValidModeFor<B2400> for Fu2 {}
impl ValidModeFor<B4800> for Fu2 {}

impl ValidModeFor<B1200> for Fu4 {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mode_commands_are_correct() {
        assert_eq!(Fu1::default().command().as_str(), "AT+FU1");
        assert_eq!(Fu2::default().command().as_str(), "AT+FU2");
        assert_eq!(Fu3::default().command().as_str(), "AT+FU3");
        assert_eq!(Fu4::default().command().as_str(), "AT+FU4");
    }
}
