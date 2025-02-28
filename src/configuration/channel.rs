use seq_macro::seq;

seq!(N in 1..=127 {
    #[derive(Debug, Clone, Copy, defmt::Format, Eq, PartialEq)]
    /// Represents a valid channel for the HC-12. 1-127 are valid channels, but 1-100 are generally reccomdned
    #[repr(u8)]
    pub enum Channel {
        // Expands to Channel1, Channel2, ..., Channel127
        #(
            Channel~N,
        )*
    }
});

impl Default for Channel {
    /// The default channel is 1
    fn default() -> Self {
        Channel::Channel1
    }
}

impl From<Channel> for u8 {
    fn from(channel: Channel) -> Self {
        channel as u8 + 1u8
    }
}

/// Invalid channel conversion error
#[derive(Debug, Clone, Copy, defmt::Format)]
pub struct InvalidChannelVariant {
    /// The invalid channel value
    pub attempted_channel: u8,
}

impl TryFrom<u8> for Channel {
    type Error = InvalidChannelVariant;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value < 1 || value > 127 {
            Err(InvalidChannelVariant {
                attempted_channel: value,
            })
        } else {
            // SAFETY: The value is in the range of valid channels, so this is safe
            Ok(unsafe { core::mem::transmute(value - 1) })
        }
    }
}
