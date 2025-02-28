use embedded_hal::{delay::DelayNs, digital::OutputPin};
use embedded_io::{Read, ReadReady, Write};

use crate::{
    configuration::{baudrates::B9600, Baudrate, HC12Configuration},
    modes::{AT, FU3},
};

use super::{ProgrammingPair, HC12};

pub struct HC12Builder<Uart, ProgrammingPin, Mode, Baud> {
    uart: Uart,
    programming: ProgrammingPin,
    mode: Mode,
    baud: Baud,
}

impl<Uart, ProgrammingPin, Mode, Baud> HC12Builder<Uart, ProgrammingPin, Mode, Baud> {
    /// Create a new empty HC12Builder.
    pub fn empty() -> HC12Builder<(), (), (), ()> {
        HC12Builder {
            uart: (),
            programming: (),
            mode: (),
            baud: (),
        }
    }

    /// Return inner attributes
    pub fn into_inner(self) -> (Uart, ProgrammingPin, Mode, Baud) {
        (self.uart, self.programming, self.mode, self.baud)
    }

    /// Add a UART to the builder
    pub fn uart<U: Read + Write + ReadReady, B: Baudrate>(
        self,
        uart: U,
        baud: B,
    ) -> HC12Builder<U, ProgrammingPin, Mode, B> {
        HC12Builder {
            uart,
            programming: self.programming,
            mode: self.mode,
            baud,
        }
    }

    /// Add an output pin to the builder
    pub fn programming_resources<P: OutputPin, D: DelayNs>(
        self,
        programming: P,
        delay: D,
    ) -> HC12Builder<Uart, ProgrammingPair<P, D>, Mode, Baud> {
        HC12Builder {
            uart: self.uart,
            programming: ProgrammingPair {
                pin: programming,
                delay,
            },
            mode: self.mode,
            baud: self.baud,
        }
    }
}

impl<Uart, ProgrammingPin, Baud> HC12Builder<Uart, ProgrammingPin, (), Baud>
where
    Uart: Read + Write,
    Baud: Baudrate,
{
    /// Add FU3 mode to the builder, this is allowed for any baudrate
    pub fn fu3(
        self,
        configuration: HC12Configuration,
    ) -> HC12Builder<Uart, ProgrammingPin, FU3<Baud>, Baud> {
        HC12Builder {
            uart: self.uart,
            programming: self.programming,
            mode: FU3::new(self.baud, configuration),
            baud: self.baud,
        }
    }
}

impl<Uart, ProgrammingPin> HC12Builder<Uart, ProgrammingPin, (), B9600>
where
    Uart: Read + Write + ReadReady,
{
    /// Add AT Mode to the builder. AT Mode requires a baudrate of 9600
    pub fn at(
        self,
        configuration: HC12Configuration,
    ) -> HC12Builder<Uart, ProgrammingPin, crate::modes::AT<B9600>, B9600> {
        HC12Builder {
            uart: self.uart,
            programming: self.programming,
            mode: AT::new(B9600, configuration),
            baud: self.baud,
        }
    }
}

impl<Uart, ProgrammingPin, Baud> HC12Builder<Uart, ProgrammingPin, FU3<Baud>, Baud>
where
    Uart: Read + Write + ReadReady,
    ProgrammingPin: OutputPin,
    Baud: Baudrate,
{
    /// Attempt to build the HC12 device
    /// this can fail if the pin fails
    pub fn attempt_build(
        self,
    ) -> Result<
        HC12<Uart, ProgrammingPin, FU3<Baud>, Baud>,
        (ProgrammingPin::Error, Uart, ProgrammingPin),
    > {
        let attr = self.into_inner();
        let uart = attr.0;
        let mut programming = attr.1;
        let mode = attr.2;
        let baud = attr.3;

        match programming.set_high() {
            Ok(()) => Ok(HC12::new(uart, programming, mode, baud)),
            Err(e) => Err((e, uart, programming)),
        }
    }
}
