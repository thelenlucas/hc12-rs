use crate::configuration::Power;

use super::Channel;

/// A configuration structure, holding the current settings of the HC-12.
/// This can by dynamically built for the non-programmable initialization of the HC-12,
/// but do so with caution
#[derive(Debug, Clone, Copy, PartialEq, Eq, defmt::Format)]
pub struct HC12Configuration {
    /// The current power level of the HC-12
    pub power: Power,
    /// The current channel of the HC-12
    pub channel: Channel,
}

impl Default for HC12Configuration {
    fn default() -> Self {
        HC12Configuration {
            power: Power::default(),
            channel: Channel::default(),
        }
    }
}
