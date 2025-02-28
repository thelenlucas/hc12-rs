use heapless::String;

use super::ATCommand;
use core::fmt::Write as _;

/// Marks a supported Baudrate for the HC-12
pub trait Baudrate: crate::sealed::Sealed + Copy + Clone {
    /// The Baudrate in bits per second, used for the host UART
    const HOST_BAUD: u32;
    /// The in-air Baudrate in bits per second, used for the HC-12
    const IN_AIR_BAUD: u32;

    /// The Baudrate in bits per second, used for the host UART
    fn host_baud(&self) -> u32 {
        Self::HOST_BAUD
    }

    /// The in-air Baudrate in bits per second, used for the HC-12
    fn in_air_baud(&self) -> u32 {
        Self::IN_AIR_BAUD
    }

    /// Command to enter the baudrate in AT mode
    fn at_command(self) -> ATCommand {
        let mut command_string = String::new();
        write!(command_string, "AT+B{}", Self::HOST_BAUD).unwrap(); // Even 115200 fits in 16 bytes
        ATCommand::from(command_string)
    }
}

/// Marks a buadrate that is supported for AT mode
pub trait ATCompatBaudrate: Baudrate {}

/// Marks a Baudrate that is supported for FU2
pub trait FU2ModeBaudrate: Baudrate {}

pub mod baudrates {
    use super::*;

    /// 1200 baud
    #[derive(Debug, Clone, Copy, defmt::Format)]
    pub struct B1200;
    impl crate::sealed::Sealed for B1200 {}
    impl Baudrate for B1200 {
        const HOST_BAUD: u32 = 1200;
        const IN_AIR_BAUD: u32 = 5000;
    }
    impl FU2ModeBaudrate for B1200 {}

    /// 2400 baud
    #[derive(Debug, Clone, Copy, defmt::Format)]
    pub struct B2400;
    impl crate::sealed::Sealed for B2400 {}
    impl Baudrate for B2400 {
        const HOST_BAUD: u32 = 2400;
        const IN_AIR_BAUD: u32 = 5000;
    }
    impl FU2ModeBaudrate for B2400 {}

    /// 4800 baud
    #[derive(Debug, Clone, Copy, defmt::Format)]
    pub struct B4800;
    impl crate::sealed::Sealed for B4800 {}
    impl Baudrate for B4800 {
        const HOST_BAUD: u32 = 4800;
        const IN_AIR_BAUD: u32 = 15000;
    }
    impl FU2ModeBaudrate for B4800 {}

    /// 9600 baud. This is the only Baudrate that is supported for AT mode.
    #[derive(Debug, Clone, Copy, defmt::Format)]
    pub struct B9600;
    impl crate::sealed::Sealed for B9600 {}
    impl Baudrate for B9600 {
        const HOST_BAUD: u32 = 9600;
        const IN_AIR_BAUD: u32 = 15000;
    }
    impl ATCompatBaudrate for B9600 {}

    /// 19200 baud
    #[derive(Debug, Clone, Copy, defmt::Format)]
    pub struct B19200;
    impl crate::sealed::Sealed for B19200 {}
    impl Baudrate for B19200 {
        const HOST_BAUD: u32 = 19200;
        const IN_AIR_BAUD: u32 = 58000;
    }

    /// 38400 baud
    #[derive(Debug, Clone, Copy, defmt::Format)]
    pub struct B38400;
    impl crate::sealed::Sealed for B38400 {}
    impl Baudrate for B38400 {
        const HOST_BAUD: u32 = 38400;
        const IN_AIR_BAUD: u32 = 58000;
    }

    /// 57600 baud
    #[derive(Debug, Clone, Copy, defmt::Format)]
    pub struct B57600;
    impl crate::sealed::Sealed for B57600 {}
    impl Baudrate for B57600 {
        const HOST_BAUD: u32 = 57600;
        const IN_AIR_BAUD: u32 = 236000;
    }

    /// 115200 baud
    #[derive(Debug, Clone, Copy, defmt::Format)]
    pub struct B115200;
    impl crate::sealed::Sealed for B115200 {}
    impl Baudrate for B115200 {
        const HOST_BAUD: u32 = 115200;
        const IN_AIR_BAUD: u32 = 236000;
    }
}
