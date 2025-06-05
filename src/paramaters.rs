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
        433_000.0 + 400.0 * self.0 as f32
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
