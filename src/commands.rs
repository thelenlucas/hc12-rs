use core::str::from_utf8;

use embedded_hal::{delay::DelayNs, digital::OutputPin};
use embedded_io::{Read, ReadReady, Write};
use heapless::String;

use crate::Error;

pub trait Command {
    fn command(&self) -> String<16>;
}

pub fn run_command<D: Read + Write + ReadReady, P: OutputPin>(
    device: &mut D,
    command: impl Command,
    delay: &mut impl DelayNs,
) -> Result<(), Error<D::Error, P::Error>> {
    device.write_all(command.command().as_bytes())?;
    device.write_all("\r\n".as_bytes())?;
    delay.delay_ms(40);

    if !device.read_ready()? {
        return Err(Error::NoResponse);
    }

    let mut buffer = [0u8; 16];
    device.read(&mut buffer)?;

    let s = from_utf8(&buffer).unwrap();

    if s.contains("Ok") {
        Ok(())
    } else {
        Err(Error::NoOk(s.try_into().unwrap()))
    }
}
