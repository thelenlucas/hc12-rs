#![no_std]

mod commands;
pub mod error;
pub mod modes;
pub mod paramaters;
pub mod speeds;

use commands::run_command;
use embedded_hal::{delay::DelayNs, digital::OutputPin};
use embedded_io::{ErrorType, Read, ReadReady, Write, WriteReady};
pub use error::*;

use modes::*;
use paramaters::{Channel, Power};
use speeds::*;

/// An HC-12 device programmer
///
/// # Example
/// ```
/// let serial = hal::serial;
/// let programming_pin = hal::gpio:Gpio1;
/// let delay = hal::delay::Timer;
///
/// let hc12 = HC12::new(serial, programming_pin, &mut delay)
///   .unwrap()
///   .speed(B9600::default())
///   .channel(Channel::new(15).unwrap())
///   .power(Power::P8)
///   .mode(Fu3::default())
///   .program(&mut timer_two)
///   .unwrap()
///   .at_mode()
///   .unwrap();
///
/// hc12.write_all("Hello, world!".as_bytes()).ok();
/// ```
pub struct HC12<Device, Pin, Mode, Speed> {
    device: Device,
    programming_pin: Pin,
    mode: Mode,
    speed: Speed,
    channel: Channel,
    power: Power,
}

impl<Device, Pin> HC12<Device, Pin, Fu3, B9600>
where
    Device: Read + Write,
    Pin: OutputPin,
{
    /// Create a new builder in programming mode. The serial port
    /// MUST be set to 9600 BPS to be able to communicate with the
    /// on-board microcontroller, in order to program properly
    pub fn new(
        device: Device,
        programming_pin: Pin,
        delay: &mut impl DelayNs,
    ) -> Result<Self, Error<Device::Error, Pin::Error>>
    where
        <Pin as embedded_hal::digital::ErrorType>::Error: embedded_io::Error,
    {
        let mut programming_pin = programming_pin;
        delay.delay_ms(40);
        if let Err(e) = programming_pin.set_low() {
            return Err(Error::PinError(e));
        }
        Ok(Self {
            device,
            programming_pin,
            mode: Fu3::default(),
            speed: B9600::default(),
            channel: Channel::default(),
            power: Power::default(),
        })
    }
}

impl<Device, Pin, Mode, Speed> HC12<Device, Pin, Mode, Speed> {
    /// Set the power of the module. The default power is the maxumum
    /// of P8
    pub fn power(self, power: Power) -> Self {
        let mut s = self;
        s.power = power;
        s
    }

    /// Set the channel. The module by default is set to Channel 0
    pub fn channel(self, channel: Channel) -> Self {
        let mut s = self;
        s.channel = channel;
        s
    }

    /// Set the mode. Factory default is `Fu3`. The mode must be allowed for the
    /// currently set speed. E.g. setting `Fu3` for a speed of `B115200` is allowed,
    /// but setting the mode to `Fu4` would not be.
    pub fn mode<NewMode>(self, mode: NewMode) -> HC12<Device, Pin, NewMode, Speed>
    where
        Speed: ValidSpeed,
        NewMode: ValidModeFor<Speed>,
    {
        HC12::<Device, Pin, NewMode, Speed> {
            device: self.device,
            programming_pin: self.programming_pin,
            mode,
            speed: self.speed,
            channel: self.channel,
            power: self.power,
        }
    }

    /// Set the speed. The speed must be valid for the current mode.
    pub fn speed<NewSpeed>(self, speed: NewSpeed) -> HC12<Device, Pin, Mode, NewSpeed>
    where
        NewSpeed: ValidSpeed,
        Mode: ValidModeFor<NewSpeed>,
    {
        HC12::<Device, Pin, Mode, NewSpeed> {
            device: self.device,
            programming_pin: self.programming_pin,
            mode: self.mode,
            speed,
            channel: self.channel,
            power: self.power,
        }
    }
}

impl<Device, Pin, Mode, Speed> HC12<Device, Pin, Mode, Speed>
where
    Device: Read + Write + ReadReady,
    Pin: OutputPin,
    Mode: ValidMode + ValidModeFor<Speed>,
    Speed: ValidSpeed,
{
    /// Program the HC12
    pub fn program(
        mut self,
        delay: &mut impl DelayNs,
    ) -> Result<ProgrammedHC12<Device, Pin>, Error<Device::Error, Pin::Error>> {
        run_command::<Device, Pin>(&mut self.device, Speed::default(), delay)?;
        run_command::<Device, Pin>(&mut self.device, Mode::default(), delay)?;
        run_command::<Device, Pin>(&mut self.device, self.power, delay)?;
        run_command::<Device, Pin>(&mut self.device, self.channel, delay)?;

        delay.delay_ms(80);

        ProgrammedHC12::new(self.device, self.programming_pin)
    }
}

/// A programmed HC-12 device. This can be used directly as a serial device,
/// or returned to AT (programming) mode, or decomposed to return the pin and the
/// serial device used in programming the module
pub struct ProgrammedHC12<Device, Pin> {
    device: Device,
    pin: Pin,
}

impl<Device, Pin> ProgrammedHC12<Device, Pin>
where
    Device: ErrorType,
    Pin: OutputPin,
{
    pub(crate) fn new(device: Device, pin: Pin) -> Result<Self, Error<Device::Error, Pin::Error>> {
        let mut pin = pin;
        let res = pin.set_high();

        match res {
            Ok(()) => Ok(Self { device, pin }),
            Err(e) => Err(Error::PinError(e)),
        }
    }

    /// Decompose the device to its serial port and programming pin
    pub fn inner(self) -> (Device, Pin) {
        (self.device, self.pin)
    }

    /// Return to programming mode. This is stateless, and the last set parameters are not
    /// persistent, but will be on the device.
    #[allow(clippy::type_complexity)]
    pub fn at_mode(
        self,
        delay: &mut impl DelayNs,
    ) -> Result<HC12<Device, Pin, Fu3, B9600>, Error<Device::Error, Pin::Error>>
    where
        Device: Read + ReadReady + Write,
        <Pin as embedded_hal::digital::ErrorType>::Error: embedded_io::Error,
    {
        HC12::new(self.device, self.pin, delay)
    }
}

impl<Device, Pin> ErrorType for ProgrammedHC12<Device, Pin>
where
    Device: ErrorType,
{
    type Error = Device::Error;
}

impl<Device, Pin> Read for ProgrammedHC12<Device, Pin>
where
    Device: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.device.read(buf)
    }
}

impl<Device, Pin> ReadReady for ProgrammedHC12<Device, Pin>
where
    Device: ReadReady,
{
    fn read_ready(&mut self) -> Result<bool, Self::Error> {
        self.device.read_ready()
    }
}

impl<Device, Pin> Write for ProgrammedHC12<Device, Pin>
where
    Device: Write,
{
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.device.write(buf)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.device.flush()
    }
}

impl<Device, Pin> WriteReady for ProgrammedHC12<Device, Pin>
where
    Device: WriteReady,
{
    fn write_ready(&mut self) -> Result<bool, Self::Error> {
        self.device.write_ready()
    }
}
