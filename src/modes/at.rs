use crate::configuration::{ATCommand, ATCommandString, Channel, Power};

use core::fmt::Debug;
use embedded_hal::delay::DelayNs;
use embedded_io::{Error, Read, ReadReady, Write};
use heapless::String;

use crate::{
    configuration::{baudrates::B9600, Baudrate, HC12Configuration},
    ValidProgrammingResources, HC12,
};

use super::ValidHC12Mode;

/// AT Mode, used for sending AT commands to configure the HC-12
#[derive(Copy, Clone)]
pub struct AT<B: Baudrate> {
    current_configuration: HC12Configuration,
    pub(crate) current_programmed_baudrate: B,
}
impl<B: Baudrate> crate::sealed::Sealed for AT<B> {}
impl<B: Baudrate> ValidHC12Mode for AT<B> {
    fn get_config(&self) -> HC12Configuration {
        self.current_configuration
    }
}
impl<B: Baudrate> AT<B> {
    pub fn new(baudrate: B, configuration: HC12Configuration) -> Self {
        AT {
            current_configuration: configuration,
            current_programmed_baudrate: baudrate,
        }
    }
}

/// An error condition of the AT mode
#[derive(Clone, Debug, defmt::Format)]
pub enum ATError<E: Error> {
    NoResponse,
    NoOK(String<16>),
    InvalidResponse,
    DeviceError(E),
}

/// An AT programming error, which returns the error, and the original state
pub struct ATProgrammingError<E: Debug + Error, U, R, M, B> {
    pub error: ATError<E>,
    pub hc12: HC12<U, R, M, B>,
}

// Implement Debug for ATProgrammingError
impl<E, U, R, M, B> Debug for ATProgrammingError<E, U, R, M, B>
where
    E: Debug + Error,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ATProgrammingError {{ error: {:?} }}", self.error)
    }
}

impl<U, R, B: Baudrate> HC12<U, R, AT<B>, B9600>
where
    U: Read + ReadReady + Write,
    R: ValidProgrammingResources + DelayNs,
{
    /// Reads a response off of the UART, up to 16 bytes.
    fn read_at_response(&mut self) -> Result<String<16>, ATError<U::Error>> {
        match self
            .uart
            .read_ready()
            .map_err(|e| ATError::DeviceError(e))?
        {
            false => Err(ATError::NoResponse),
            true => {
                let mut buf = [0u8; 16];
                let len = self
                    .uart
                    .read(&mut buf)
                    .map_err(|e| ATError::DeviceError(e))?;
                let mut response = String::<16>::new();
                for byte in &buf[0..len] {
                    response.push(*byte as char).unwrap(); // We know this is safe
                }

                Ok(response)
            }
        }
    }

    /// Clear the buffer
    fn clear_buffer(&mut self) -> Result<(), ATError<U::Error>> {
        while self
            .uart
            .read_ready()
            .map_err(|e| ATError::DeviceError(e))?
        {
            let mut buf = [0u8; 1];
            self.uart
                .read(&mut buf)
                .map_err(|e| ATError::DeviceError(e))?;
        }

        Ok(())
    }

    /// If the buffer contains "OK", the last command was successful. If the command was good, this also returns
    /// up to 16 bytes of the response.
    fn check_ok(&mut self) -> Result<String<16>, ATError<U::Error>> {
        let res = self.read_at_response()?;
        if res.contains("OK") {
            self.clear_buffer()?;
            Ok(res)
        } else {
            Err(ATError::NoOK(res))
        }
    }

    /// Send an AT command to the HC-12
    fn send_at_command(&mut self, command: ATCommand) -> Result<(), ATError<U::Error>> {
        let command_string = ATCommandString::from(command);
        if let Err(e) = self.uart.write(command_string.as_bytes()) {
            return Err(ATError::DeviceError(e));
        }

        Ok(())
    }

    /// Send an AT command to the HC-12, and wait for a response. Allow for a timeout of up to 100ms
    fn at_command_sequence(&mut self, command: ATCommand) -> Result<(), ATError<U::Error>> {
        // Clear buffer
        self.clear_buffer()?;
        self.send_at_command(command)?;
        // Delay for 100ms
        self.programming.delay_ms(100);
        self.check_ok()?;
        Ok(())
    }
}

impl<U, R, B: Baudrate> HC12<U, R, AT<B>, B9600>
where
    U: Read + ReadReady + Write,
    R: ValidProgrammingResources + DelayNs,
{
    /// Set the baudrate of the HC-12 (for the moment all baudrates are supported)
    pub fn set_baudrate<N: Baudrate>(
        mut self,
        baudrate: N,
    ) -> Result<HC12<U, R, AT<N>, B9600>, ATProgrammingError<U::Error, U, R, AT<B>, B9600>> {
        let command = baudrate.at_command();
        defmt::info!("COMMAND: {:?}", command);
        match self.at_command_sequence(command) {
            Ok(_) => {
                let inner = self.into_inner();
                let old_programmer = inner.1;
                let old_mode = inner.2;
                let old_config = old_mode.get_config();

                Ok(HC12::new(
                    inner.0,
                    old_programmer,
                    AT::new(baudrate, old_config),
                    B9600,
                ))
            }
            Err(e) => Err(ATProgrammingError {
                error: e,
                hc12: self,
            }),
        }
    }

    /// Set the channel of the HC-12
    pub fn set_channel(
        mut self,
        channel: Channel,
    ) -> Result<HC12<U, R, AT<B>, B9600>, ATProgrammingError<U::Error, U, R, AT<B>, B9600>> {
        let command = channel.into();
        defmt::info!("COMMAND: {:?}", command);
        match self.send_at_command(command) {
            Ok(_) => Ok(self),
            Err(e) => Err(ATProgrammingError {
                error: e,
                hc12: self,
            }),
        }
    }

    /// Sets the power of the HC-12
    pub fn set_power(
        mut self,
        power: Power,
    ) -> Result<HC12<U, R, AT<B>, B9600>, ATProgrammingError<U::Error, U, R, AT<B>, B9600>> {
        let command: ATCommand = power.into();
        defmt::info!("COMMAND: {:?}", command);
        match self.at_command_sequence(command) {
            Ok(_) => Ok(self),
            Err(e) => Err(ATProgrammingError {
                error: e,
                hc12: self,
            }),
        }
    }
}
