use embedded_io::{ErrorType, Read, ReadReady, Write, WriteReady};

use crate::{configuration::HC12Configuration, sealed::Sealed};

/// The HC-12 device
pub struct HC12<U, M, P, B> {
    /// The underlying UART device
    pub(crate) uart: U,
    /// The mode of the device
    pub(crate) mode: M,
    /// The programming resources
    pub(crate) programming: P,
    /// The speed of the current UART
    pub(crate) speed: B,
    /// The configuration of the device
    pub(crate) configuration: HC12Configuration, // Todo
}
impl<U, M, P, B> Sealed for HC12<U, M, P, B> {}
impl<U, M, P, B> HC12<U, M, P, B> {
    /// Create a new builder
    pub fn builder() -> builder::HC12Builder<(), (), ()> {
        builder::HC12Builder::new()
    }
}

/// While in transparent, if the UART has an error type, we can use it to pass through Read + Write + ReadReady + WriteReady
impl<U, M, P, B> ErrorType for HC12<U, M, P, B>
where
    U: ErrorType,
{
    type Error = U::Error;
}

impl<U, M, P, B> Read for HC12<U, M, P, B>
where
    U: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.uart.read(buf)
    }
}

impl<U, M, P, B> Write for HC12<U, M, P, B>
where
    U: Write,
{
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.uart.write(buf)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.uart.flush()
    }
}

impl<U, M, P, B> ReadReady for HC12<U, M, P, B>
where
    U: ReadReady,
{
    fn read_ready(&mut self) -> Result<bool, Self::Error> {
        self.uart.read_ready()
    }
}

impl<U, M, P, B> WriteReady for HC12<U, M, P, B>
where
    U: WriteReady,
{
    fn write_ready(&mut self) -> Result<bool, Self::Error> {
        self.uart.write_ready()
    }
}

pub mod builder {
    use core::fmt::Debug;

    use defmt::Format;
    use embedded_hal::{delay::DelayNs, digital::OutputPin};
    use embedded_io::{Read, ReadReady, Write};

    use crate::{
        baudrates::{ValidBaudrate, B9600},
        configuration::HC12Configuration,
        modes::{UnkownTransparentMode, ValidTransparentMode, AT, FU3},
        programming::{HC12ProgrammingError, ProgrammingResouces, ValidProgrammingResources},
    };

    use super::*;

    /// A builder for the HC-12 device
    pub struct HC12Builder<U, P, B> {
        uart: U,
        programming: P,
        speed: B,
        configuration: (),
    }
    impl<U, P, B> Sealed for HC12Builder<U, P, B> {}

    impl HC12Builder<(), (), ()> {
        pub fn new() -> Self {
            HC12Builder {
                uart: (),
                programming: (),
                speed: (),
                configuration: (),
            }
        }
    }

    impl<U, P, B> HC12Builder<U, P, B> {
        pub fn to_inner(self) -> (U, P, B) {
            (self.uart, self.programming, self.speed)
        }

        /// Add a serial device and a speed to the device
        pub fn serial<UART, Baud>(
            self,
            new_uart: UART,
            new_speed: Baud,
        ) -> HC12Builder<UART, P, Baud>
        where
            UART: Read + Write,
            Baud: ValidBaudrate,
        {
            HC12Builder {
                uart: new_uart,
                programming: self.programming,
                speed: new_speed,
                configuration: self.configuration,
            }
        }

        /// Add programming resources to the device
        pub fn programming_resources<Resources: ValidProgrammingResources>(
            self,
            resources: Resources,
        ) -> HC12Builder<U, Resources, B> {
            HC12Builder {
                uart: self.uart,
                programming: resources,
                speed: self.speed,
                configuration: self.configuration,
            }
        }

        pub fn programming_resources_unpaired<Pin: OutputPin, Delay: DelayNs>(
            self,
            pin: Pin,
            delay: Delay,
        ) -> HC12Builder<U, ProgrammingResouces<Pin, Delay>, B> {
            let resources = ProgrammingResouces::new(pin, delay);
            self.programming_resources(resources)
        }
    }

    /// When a UART device is added, we may transition into any transparent mode. This requires the user to verify their device is set up properly,
    /// and the device will not be able to enter programming mode if the programming resources are not provided, but is infallible
    impl<U, P, B> HC12Builder<U, P, B>
    where
        U: Read + Write,
        B: ValidBaudrate,
    {
        /// Build infallibly into FU3 mode
        pub fn into_fu3(self, configuration: HC12Configuration) -> HC12<U, FU3<B>, P, B>
        where
            FU3<B>: ValidTransparentMode,
        {
            let speed = self.speed;
            HC12 {
                uart: self.uart,
                mode: FU3::new(speed),
                programming: self.programming,
                speed,
                configuration,
            }
        }
    }

    impl<U, P> HC12Builder<U, P, B9600>
    where
        U: Read + Write + ReadReady,
        P: ValidProgrammingResources,
    {
        /// When a programming device is added, we can attempt to build into AT mode, which is fallible due to the reliance. If this fails,
        /// it returns the *old* mode, so you can attempt an infallible transition into a transparent mode, or a retry, depending
        /// on your needs
        pub fn try_build_at<B: ValidBaudrate>(
            mut self,
            programmed_baudrate: B,
            configuration: HC12Configuration,
        ) -> Result<
            HC12<U, AT<B, UnkownTransparentMode>, P, B9600>,
            HC12ProgrammingError<P::Error, Self>,
        > {
            if let Err(e) = self.programming.pull_at() {
                return Err(HC12ProgrammingError {
                    error: e,
                    last: self,
                });
            }
            let speed = self.speed;
            Ok(HC12 {
                uart: self.uart,
                mode: AT::new(UnkownTransparentMode, programmed_baudrate),
                programming: self.programming,
                speed,
                configuration,
            })
        }
    }
}
