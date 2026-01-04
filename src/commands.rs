use core::str::from_utf8;

use embedded_hal::delay::DelayNs;
use embedded_io::{Read, ReadReady, Write};
use heapless::String;

use crate::Error;

pub trait Command {
    fn command(&self) -> String<16>;
}

pub(crate) fn run_command<D: Read + Write + ReadReady>(
    device: &mut D,
    command: impl Command,
    delay: &mut impl DelayNs,
) -> Result<(), Error<D::Error>> {
    send_command(device, command)?;
    delay.delay_ms(40);
    recieve_command(device)?;
    Ok(())
}

fn send_command<D: Write>(device: &mut D, command: impl Command) -> Result<(), D::Error> {
    device.write_all(command.command().as_bytes())?;
    device.write_all("\r\n".as_bytes())?;
    Ok(())
}

fn recieve_command<D: Read + ReadReady>(device: &mut D) -> Result<(), Error<D::Error>> {
    // Check if device has data ready before attempting to read
    if !device.read_ready().map_err(Error::DeviceError)? {
        return Err(Error::NoResponse);
    }

    let mut buffer = [0u8; 16];
    let mut pointer = 0;

    while let Ok(bytes) = device.read(&mut buffer[pointer..]) {
        if bytes == 0 {
            break;
        }
        pointer += bytes;

        if buffer.contains(&b'\n') {
            break;
        }
    }

    // If we didn't read anything, return NoResponse error
    if pointer == 0 {
        return Err(Error::NoResponse);
    }

    let s = from_utf8(&buffer[..pointer]).unwrap();
    if s.contains("OK") {
        Ok(())
    } else {
        Err(Error::NoOK(s.try_into().unwrap()))
    }
}

#[cfg(test)]
mod test {
    use crate::speeds::B9600;

    use super::*;
    use embedded_hal_mock::eh1 as hal;
    use embedded_io::ErrorType;
    use mock_embedded_io as io;

    #[test]
    fn send_b9600() {
        let expected_command = "AT+B9600\r\n".as_bytes();
        let mut writer = io::Sink::new().accept_data(expected_command.len());
        send_command(&mut writer, B9600::default()).unwrap();
        assert_eq!(expected_command, writer.into_inner_data());
    }

    #[test]
    fn recieve_b9600() {
        struct ReadySource {
            src: io::Source,
        }

        impl embedded_io::ErrorType for ReadySource {
            type Error = io::MockError;
        }

        impl embedded_io::Read for ReadySource {
            fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
                self.src.read(buf)
            }
        }

        impl embedded_io::ReadReady for ReadySource {
            fn read_ready(&mut self) -> Result<bool, Self::Error> {
                Ok(true) // Always ready for test
            }
        }

        let response = "OK+B9600\r\n".as_bytes();
        let mut reader = ReadySource {
            src: io::Source::new().data(response),
        };
        recieve_command(&mut reader).unwrap()
    }

    #[test]
    fn receive_non_ok_response() {
        struct ReadySource {
            src: io::Source,
        }

        impl embedded_io::ErrorType for ReadySource {
            type Error = io::MockError;
        }

        impl embedded_io::Read for ReadySource {
            fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
                self.src.read(buf)
            }
        }

        impl embedded_io::ReadReady for ReadySource {
            fn read_ready(&mut self) -> Result<bool, Self::Error> {
                Ok(true) // Always ready for test
            }
        }

        let response = b"ERR+CMD\r\n";
        let mut reader = ReadySource {
            src: io::Source::new().data(response),
        };
        let err = recieve_command(&mut reader).unwrap_err();
        // We get a NoOK variant
        if let Error::NoOK(s) = err {
            assert!(s.as_str().starts_with("ERR+CMD"));
        } else {
            panic!("Expected Error::NoOK, got {:?}", err);
        }
    }

    #[test]
    fn run_command_happy_path() {
        // Combine a Sink and Source into a single device...
        struct Duo {
            sink: io::Sink,
            src: io::Source,
        }

        impl ErrorType for Duo {
            type Error = mock_embedded_io::MockError;
        }

        impl embedded_io::Write for Duo {
            fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
                self.sink.write(buf)
            }
            fn flush(&mut self) -> Result<(), Self::Error> {
                Ok(())
            }
        }
        impl embedded_io::Read for Duo {
            fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
                self.src.read(buf)
            }
        }
        impl embedded_io::ReadReady for Duo {
            fn read_ready(&mut self) -> Result<bool, Self::Error> {
                Ok(true) // Always ready for test
            }
        }

        // Prepare a device that will accept a B9600 command and then return OK
        let mut dev = Duo {
            sink: io::Sink::new().accept_data(8 + 2), // "AT+B9600" + "\r\n"
            src: io::Source::new().data(b"OK+B9600\r\n"),
        };
        let mut delay = hal::delay::NoopDelay::new();
        // Should succeed without error
        run_command(&mut dev, B9600::default(), &mut delay).unwrap();
    }

    #[test]
    fn receive_no_response_when_not_ready() {
        struct NotReadySource {
            src: io::Source,
        }

        impl embedded_io::ErrorType for NotReadySource {
            type Error = io::MockError;
        }

        impl embedded_io::Read for NotReadySource {
            fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
                self.src.read(buf)
            }
        }

        impl embedded_io::ReadReady for NotReadySource {
            fn read_ready(&mut self) -> Result<bool, Self::Error> {
                Ok(false) // Not ready - simulate no response available
            }
        }

        let mut reader = NotReadySource {
            src: io::Source::new().data(b""),
        };
        let err = recieve_command(&mut reader).unwrap_err();

        // We should get a NoResponse error
        if !matches!(err, Error::NoResponse) {
            panic!("Expected Error::NoResponse, got {:?}", err);
        }
    }
}
