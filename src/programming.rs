use defmt::Format;
use embedded_hal::{
    delay::DelayNs,
    digital::{ErrorType, OutputPin},
};
use embedded_io::{Error, ErrorType as IOErrorType, Read, ReadReady, Write};

use crate::{
    baudrates::{ValidBaudrate, B9600},
    configuration::{Channel, HC12Configuration, Power},
    device::HC12,
    modes::{ModeValidWithBaudrate, ValidTransparentMode, AT, FU3},
    sealed::Sealed,
};

use core::fmt::{Debug, Write as _};

/// Valid programming resources
pub trait ValidProgrammingResources: Sealed + ErrorType {
    fn pull_at(&mut self) -> Result<(), Self::Error>;
    fn pull_transparent(&mut self) -> Result<(), Self::Error>;
    fn wait_for_response(&mut self);
}

/// Underlying resources for programming the HC12. These are optional unless you want to go into AT mode, or change the device configuration in any way
pub struct ProgrammingResouces<P: OutputPin, D: DelayNs> {
    /// The programming pin
    pub programming_pin: P,
    /// The delay timer
    pub delay: D,
}
impl<P: OutputPin, D: DelayNs> Sealed for ProgrammingResouces<P, D> {}
impl<P: OutputPin, D: DelayNs> ValidProgrammingResources for ProgrammingResouces<P, D> {
    /// To enter AT mode, pull the programming pin low, and delay 100ms
    fn pull_at(&mut self) -> Result<(), P::Error> {
        self.programming_pin.set_low()?;
        self.wait_for_response();
        Ok(())
    }

    /// To exit AT mode, pull the programming pin high, and delay 100ms
    fn pull_transparent(&mut self) -> Result<(), P::Error> {
        self.programming_pin.set_high()?;
        self.wait_for_response();
        Ok(())
    }

    /// Wait for 100ms
    fn wait_for_response(&mut self) {
        self.delay.delay_ms(100);
    }
}
impl<P: OutputPin, D: DelayNs> ProgrammingResouces<P, D> {
    /// Create a new programming resources
    pub fn new(programming_pin: P, delay: D) -> Self {
        ProgrammingResouces {
            programming_pin,
            delay,
        }
    }
}
impl<P: OutputPin, D: DelayNs> ErrorType for ProgrammingResouces<P, D> {
    type Error = P::Error;
}

/// A failed programming of the HC-12 device. This returns the the original device, so the user can attempt to build again,
/// and reimpliments debug and format to tunnel into the underlying error
pub struct HC12ProgrammingError<E: Debug, LastState> {
    /// Error that caused the build to fail
    pub error: E,
    /// The builder that experienced the error
    pub last: LastState,
}
impl<E: core::fmt::Debug, LastState> core::fmt::Debug for HC12ProgrammingError<E, LastState> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.error.fmt(f)
    }
}

/// A string to or from the AT module
pub type ATCommandString = heapless::String<16>;

/// No OK was received
#[derive(Clone, Format, Debug)]
pub struct NoOkResponse {
    pub command: ATCommandString,
    pub response: ATCommandString,
}

/// An error in programming the HC12
#[derive(Clone, Format)]
pub enum ATCommandError<E: Error> {
    /// No OK was received from the AT module, but some response was received
    NoOkResponse(ATCommandString),
    /// No response was received from the AT module
    NoResponse(NoOkResponse),
    /// An error was received from the underlying UART
    UartError,
    /// A device error, from an embedded-io device
    DeviceError(E),
}
impl<E: Error> Debug for ATCommandError<E> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ATCommandError::NoOkResponse(s) => write!(f, "No OK response, got: {:?}", s),
            ATCommandError::NoResponse(e) => e.fmt(f),
            ATCommandError::UartError => write!(f, "UART error"),
            ATCommandError::DeviceError(e) => write!(f, "Device error: {:?}", e),
        }
    }
}

/// An HC-12 device in programming mode, with a valid pair of resources, and a read + readready + write device
/// may be programmed to change it's parameters
impl<U, PB, M, P> HC12<U, AT<PB, M>, P, B9600>
where
    U: Read + Write + ReadReady,
    P: ValidProgrammingResources,
    PB: ValidBaudrate,
    M: ValidTransparentMode,
{
    // Send a string to the AT module
    fn send_at_string(&mut self, string: ATCommandString) -> Result<(), ATCommandError<U::Error>> {
        // Write the string
        self.uart
            .write_all(string.as_bytes())
            .map_err(|_| ATCommandError::UartError)?;
        self.uart.flush().map_err(|_| ATCommandError::UartError)?;
        Ok(())
    }

    // Read up to 16 bytes from the AT module
    fn read_response(&mut self) -> Result<ATCommandString, ATCommandError<U::Error>> {
        if !self
            .uart
            .read_ready()
            .map_err(|e| ATCommandError::DeviceError(e))?
        {
            return Err(ATCommandError::NoResponse(NoOkResponse {
                command: ATCommandString::new(),
                response: ATCommandString::new(),
            }));
        }

        // Read up to 16 bytes
        let mut buffer = [0u8; 16];
        let read = self
            .uart
            .read(&mut buffer)
            .map_err(|_| ATCommandError::UartError)?;

        let mut response = ATCommandString::new();
        for i in 0..read {
            response.push(buffer[i] as char).ok();
        }
        Ok(response)
    }

    // Clear the buffer
    fn clear_buffer(&mut self) {
        while self.uart.read_ready().unwrap_or(false) {
            let mut buffer = [0u8; 16];
            self.uart.read(&mut buffer).ok();
        }
    }

    // Check for an OK response
    fn check_response(&mut self) -> Result<(), ATCommandError<U::Error>> {
        // Wait 100ms for a response, the HC-12 is pretty sluggish in AT mode
        self.programming.wait_for_response();

        // Read response
        let buf = self.read_response()?;

        // Check for OK
        match buf.contains("OK") {
            true => Ok(()),
            false => Err(ATCommandError::NoOkResponse(buf)),
        }
    }

    // Send a command sequence, clearing the buffer, sending the command, and checking the response
    fn send_sequence(&mut self, command: ATCommandString) -> Result<(), ATCommandError<U::Error>> {
        self.clear_buffer();
        self.send_at_string(command)?;
        self.programming.wait_for_response();
        self.check_response()
    }

    /// Set the channel of the device. This can fail, but isn't fatal, and if the programming doesn't go through,
    /// the configuration just won't be updated
    pub fn set_power(&mut self, power: Power) -> Result<(), ATCommandError<U::Error>> {
        // Create an AT string - we assume no_std
        let mut command = ATCommandString::new();
        let power: u8 = power.into();
        write!(command, "AT+P{}\r\n", power).ok();
        self.send_sequence(command)
    }

    /// Set the channel of the device. This can fail, but isn't fatal, and if the programming doesn't go through,
    /// the configuration just won't be updated
    pub fn set_channel(&mut self, channel: Channel) -> Result<(), ATCommandError<U::Error>> {
        // Create an AT string - we assume no_std
        let mut command = ATCommandString::new();
        let channel: u8 = channel.into();
        // Make sure to use three digits
        write!(command, "AT+C{:03}\r\n", channel).ok();
        self.send_sequence(command)
    }

    /// Set the device's configuration. This can fail, but isn't fatal, and if the programming doesn't go through,
    /// the configuration just won't be updated
    pub fn set_configuration(
        &mut self,
        configuration: HC12Configuration,
    ) -> Result<(), ATCommandError<U::Error>> {
        self.set_power(configuration.power)?;
        self.set_channel(configuration.channel)
    }

    // Change the programmed baudrate of the device. This can fail, and *is* fatal, so in the case of a failure,
    // we return the old state of the device to avoid having to rebuild the device. In the event of success, we
    // return the new state of the device
    pub fn set_baudrate<N: ValidBaudrate>(
        mut self,
        _new_baudrate: N,
    ) -> Result<HC12<U, AT<N, M>, P, B9600>, HC12ProgrammingError<ATCommandError<U::Error>, Self>>
    where
        M: ModeValidWithBaudrate<N> + ValidTransparentMode,
    {
        // Verify the transition first
        let mut command = ATCommandString::new();
        // AT+B{new_baudrate}\r\n
        write!(command, "AT+B{}\r\n", N::BAUDRATE).ok();
        match self.send_sequence(command) {
            Ok(_) => {
                let new_programmed_mode = M::from_baudrate(N::default());
                let new_at_mode = AT::new(new_programmed_mode, N::default());
                Ok(HC12 {
                    uart: self.uart,
                    mode: new_at_mode,
                    programming: self.programming,
                    speed: B9600,
                    configuration: self.configuration,
                })
            }

            Err(e) => Err(HC12ProgrammingError {
                error: e,
                last: self,
            }),
        }
    }

    /// Change the programmed mode of the device. This can fail, and is more destructive then configuration changes
    pub fn set_mode<N: ValidTransparentMode>(
        mut self,
        _new_mode: N,
    ) -> Result<HC12<U, AT<PB, N>, P, B9600>, HC12ProgrammingError<ATCommandError<U::Error>, Self>>
    where
        N: ModeValidWithBaudrate<PB>,
    {
        // Verify transition
        // AT+{N::CODE}\r\n
        let mut command = ATCommandString::new();
        write!(command, "AT+{}\r\n", N::CODE).ok();
        match self.send_sequence(command) {
            Ok(()) => {
                let new_programmed_mode = N::from_baudrate(self.mode.programmed_baudrate());
                let new_at_mode = AT::new(new_programmed_mode, self.mode.programmed_baudrate());
                Ok(HC12 {
                    uart: self.uart,
                    mode: new_at_mode,
                    programming: self.programming,
                    speed: B9600,
                    configuration: self.configuration,
                })
            }

            Err(e) => Err(HC12ProgrammingError {
                error: e,
                last: self,
            }),
        }
    }

    /// Go to transparent mode. This only can be done if the the mode and baudrate are a valid pair
    /// This can fail on the underlying pin, like the other programming functions
    pub fn into_transparent(
        mut self,
    ) -> Result<HC12<U, M, P, B9600>, HC12ProgrammingError<P::Error, Self>>
    where
        M: ModeValidWithBaudrate<PB>,
    {
        match self.programming.pull_transparent() {
            Ok(_) => Ok(HC12 {
                uart: self.uart,
                mode: self.mode.programmed_mode(),
                programming: self.programming,
                speed: B9600,
                configuration: self.configuration,
            }),

            Err(e) => Err(HC12ProgrammingError {
                error: e,
                last: self,
            }),
        }
    }
}
