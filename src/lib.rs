#![no_std]

mod commands;
pub mod error;
pub mod modes;
pub mod paramaters;
pub mod speeds;

use commands::run_command;
use embedded_hal::{delay::DelayNs, digital::OutputPin};
use embedded_io::{Read, ReadReady, Write};
pub use error::*;

use modes::*;
use paramaters::{Channel, Power};
use speeds::*;

/// An HC-12 device builder
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
    /// Create a new builder in programming mode
    pub fn new(
        device: Device,
        programming_pin: Pin,
        delay: &mut impl DelayNs,
    ) -> Result<Self, Error<Pin::Error>>
    where
        <Pin as embedded_hal::digital::ErrorType>::Error: embedded_io::Error,
    {
        let mut programming_pin = programming_pin;
        delay.delay_ms(40);
        programming_pin.set_low()?;
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
    /// Set the power
    pub fn power(self, power: Power) -> Self {
        let mut s = self;
        s.power = power;
        s
    }

    /// Set the channel
    pub fn channel(self, channel: Channel) -> Self {
        let mut s = self;
        s.channel = channel;
        s
    }

    /// Set the mode
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

    /// Set the speed
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
    ) -> Result<ProgrammedHC12<Device, Pin>, Error<Device::Error>> {
        run_command(&mut self.device, Speed::default(), delay)?;
        run_command(&mut self.device, Mode::default(), delay)?;
        run_command(&mut self.device, self.power, delay)?;
        run_command(&mut self.device, self.channel, delay)?;

        Ok(ProgrammedHC12::new(self.device, self.programming_pin))
    }
}

pub struct ProgrammedHC12<Device, Pin> {
    device: Device,
    pin: Pin,
}

impl<Device, Pin> ProgrammedHC12<Device, Pin> {
    pub(crate) fn new(device: Device, pin: Pin) -> Self {
        Self { device, pin }
    }

    pub(crate) fn inner(self) -> (Device, Pin) {
        (self.device, self.pin)
    }

    #[allow(clippy::type_complexity)]
    pub fn at_mode(mut self) -> Result<(Device, Pin), Error<Pin::Error>>
    where
        Pin: OutputPin,
        <Pin as embedded_hal::digital::ErrorType>::Error: embedded_io::Error,
    {
        let res = self.pin.set_high();

        match res {
            Ok(()) => Ok(self.inner()),
            Err(e) => Err(e.into()),
        }
    }
}
