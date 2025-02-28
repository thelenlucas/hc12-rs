// Iterator definitions are useful, since we have 127 configuration options
use crate::sealed::Sealed;
use defmt::Format;

/// An invalid channel
#[derive(Copy, Clone, Debug, PartialEq, Eq, Format)]
pub struct InvalidChannelSetting(u8);

/// A valid channel for the HC-12. 1-127 are allowed, but only 1-100 are reccomended
#[derive(Copy, Clone, Debug, PartialEq, Eq, Format)]
pub struct Channel {
    channel: u8,
}

impl Default for Channel {
    fn default() -> Self {
        Channel { channel: 1 }
    }
}

impl TryFrom<u8> for Channel {
    type Error = InvalidChannelSetting;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value < 1 || value > 127 {
            Err(InvalidChannelSetting(value))
        } else {
            Ok(Channel { channel: value })
        }
    }
}
impl From<Channel> for u8 {
    fn from(value: Channel) -> Self {
        value.channel
    }
}

/// A valid power level for the HC-12
#[derive(Copy, Clone, Debug, PartialEq, Eq, Format)]
pub enum Power {
    P1,
    P2,
    P3,
    P4,
    P5,
    P6,
    P7,
    P8,
}
impl Sealed for Power {}
impl Default for Power {
    /// Factory settings
    fn default() -> Self {
        Power::P5
    }
}
impl Power {
    #[allow(non_snake_case)]
    pub fn dBm(&self) -> u8 {
        match self {
            Power::P1 => 1,
            Power::P2 => 3,
            Power::P3 => 5,
            Power::P4 => 7,
            Power::P5 => 10,
            Power::P6 => 12,
            Power::P7 => 15,
            Power::P8 => 20,
        }
    }
}

/// Bad power
#[derive(Copy, Clone, Debug, PartialEq, Eq, Format)]
pub struct InvalidPowerSetting(u8);

impl TryFrom<u8> for Power {
    type Error = InvalidPowerSetting;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Power::P1),
            2 => Ok(Power::P2),
            3 => Ok(Power::P3),
            4 => Ok(Power::P4),
            5 => Ok(Power::P5),
            6 => Ok(Power::P6),
            7 => Ok(Power::P7),
            8 => Ok(Power::P8),
            _ => Err(InvalidPowerSetting(value)),
        }
    }
}

impl From<Power> for u8 {
    fn from(value: Power) -> Self {
        match value {
            Power::P1 => 1,
            Power::P2 => 2,
            Power::P3 => 3,
            Power::P4 => 4,
            Power::P5 => 5,
            Power::P6 => 6,
            Power::P7 => 7,
            Power::P8 => 8,
        }
    }
}

/// HC-12 Configuration. Consists of a channel and a power level. These configuraion parameters are non-essential, and won't dissalow
/// the module from working, like a mode failure would be, so they're not baked into the type system like the modes are.
#[derive(Copy, Clone, Debug, Format)]
pub struct HC12Configuration {
    pub channel: Channel,
    pub power: Power,
}

impl Default for HC12Configuration {
    fn default() -> Self {
        HC12Configuration {
            channel: Channel::default(),
            power: Power::default(),
        }
    }
}
