use super::{Channel, Power};
use core::fmt::Write as _;

/// AT+B115200\r\n is the longest AT command
/// Despite this, we allocate 16 bytes, to allow for some extra space for extentions
pub(crate) type ATCommandString = heapless::String<16>;

/// An AT Command string
#[derive(Debug, Clone, defmt::Format)]
pub struct ATCommand {
    /// The command string
    command: ATCommandString,
}

impl From<ATCommandString> for ATCommand {
    fn from(command: ATCommandString) -> Self {
        ATCommand { command }
    }
}

impl From<ATCommand> for ATCommandString {
    fn from(command: ATCommand) -> Self {
        command.command
    }
}

impl From<Channel> for ATCommand {
    fn from(channel: Channel) -> Self {
        let mut command_string = ATCommandString::new();
        write!(command_string, "AT+C{:03}\r\n", u8::from(channel)).unwrap(); // Make sure 3 digits are written
        ATCommand::from(command_string)
    }
}

impl From<Power> for ATCommand {
    fn from(power: Power) -> Self {
        let mut command_string = ATCommandString::new();
        let pow = match power {
            Power::P1 => 1,
            Power::P2 => 2,
            Power::P3 => 3,
            Power::P4 => 4,
            Power::P5 => 5,
            Power::P6 => 6,
            Power::P7 => 7,
            Power::P8 => 8,
        };
        write!(command_string, "AT+P{}\r\n", pow).unwrap();
        ATCommand::from(command_string)
    }
}
