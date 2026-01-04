use core::fmt::Write;
use heapless::String;

use crate::commands::Command;

/// A channel - channels between 1 and 127 are valid
#[derive(Debug)]
pub struct Channel(u8);

/// A bad channel was attempted to be created
#[derive(Debug)]
pub struct BadChannel(u8);

impl From<Channel> for u8 {
    fn from(value: Channel) -> Self {
        value.0
    }
}

impl From<BadChannel> for u8 {
    fn from(value: BadChannel) -> Self {
        value.0
    }
}

impl From<u8> for BadChannel {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl Default for Channel {
    fn default() -> Self {
        Channel(1)
    }
}

impl Command for Channel {
    fn command(&self) -> heapless::String<16> {
        let mut s = String::new();

        write!(&mut s, "AT+C{:03}", self.0).ok();

        s
    }
}

impl Channel {
    /// Try to create a channel with a u8
    pub fn new(channel: u8) -> Result<Self, BadChannel> {
        if channel < 128 && channel > 0 {
            Ok(Self(channel))
        } else {
            Err(channel.into())
        }
    }

    /// Get the frequency of the channel, in  MHz
    pub fn mhz(&self) -> f32 {
        433.0 + 0.4 * self.0 as f32
    }

    /// Get the frequency of the channel, in KHz
    pub fn khz(&self) -> u32 {
        433_000 + 400 * self.0 as u32
    }
}

impl TryFrom<u8> for Channel {
    type Error = BadChannel;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

/// A valid power level
#[repr(u8)]
#[derive(Default, Debug, Clone, Copy)]
pub enum Power {
    P1 = 1,
    P2 = 2,
    P3 = 3,
    P4 = 4,
    P5 = 5,
    P6 = 6,
    P7 = 7,
    #[default]
    P8 = 8,
}

impl From<&Power> for u8 {
    fn from(value: &Power) -> Self {
        *value as u8
    }
}

impl Command for Power {
    fn command(&self) -> String<16> {
        let mut s = String::new();
        let p: u8 = self.into();
        write!(&mut s, "AT+P{}", p).ok();
        s
    }
}

impl Power {
    /// Power of the modules in dBm
    pub fn power_decible_milliwatts(&self) -> i8 {
        match self {
            Power::P1 => -1,
            Power::P2 => 2,
            Power::P3 => 5,
            Power::P4 => 8,
            Power::P5 => 11,
            Power::P6 => 14,
            Power::P7 => 17,
            Power::P8 => 20,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn channel_new_valid() {
        assert!(Channel::new(1).is_ok());
        assert!(Channel::new(127).is_ok());
    }

    #[test]
    fn channel_new_invalid() {
        assert!(Channel::new(0).is_err());
        assert!(Channel::new(128).is_err());
    }

    #[test]
    fn channel_default_is_1() {
        let default: Channel = Channel::default();
        assert_eq!(u8::from(default), 1);
    }

    #[test]
    fn channel_command_format() {
        let ch = Channel::new(5).unwrap();
        // zero-padded three-digit decimal
        assert_eq!(ch.command().as_str(), "AT+C005");
    }

    #[test]
    fn channel_mhz_calculation() {
        let ch = Channel::new(10).unwrap();
        let expected = 433.0 + 0.4 * 10.0;
        assert_eq!(ch.mhz(), expected);
    }

    #[test]
    fn channel_try_from() {
        assert!(Channel::try_from(127).is_ok());
        assert!(Channel::try_from(200).is_err());
    }

    #[test]
    fn power_variants_and_default() {
        // Explicit variant
        assert_eq!(Power::P3.command().as_str(), "AT+P3");
        // Default is P8
        assert_eq!(Power::default().command().as_str(), "AT+P8");
    }
}
