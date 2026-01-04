#![cfg_attr(not(all(test, feature = "std")), no_std)]

mod commands;
pub mod error;
pub mod modes;
pub mod paramaters;
pub mod speeds;

use core::marker::PhantomData;

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
/// ```ignore
/// let mut serial = hal::serial;
/// let mut programming_pin = hal::gpio::Gpio1;
/// let mut delay = hal::delay::Timer;
///
/// let mut hc12 = HC12::factor_settings(&mut serial, &mut programming_pin, &mut delay)
///   .unwrap()
///   .channel(Channel::new(15).unwrap())
///   .power(Power::P8)
///   .b4800()
///   .fu3()
///   .program(&mut timer_two)
///   .unwrap()
///   .into_transparent_mode(&mut delay)
///   .unwrap();
///
/// hc12.write_all("Hello world!".as_bytes()).ok();
///
/// let mut hc12_low_power = hc12.into_programming_mode(&mut delay)
///     .unwrap()
///     .fu1()
///     .program(&mut delay)
///     .unwrap()
///     .into_transparent_mode(&mut delay)
///     .unwrap();
///
/// hc12_low_power.write_all(b"Hello from the low power mode!").ok();
/// ```
pub struct HC12<'a, Device, Pin, Mode, Speed> {
    device: &'a mut Device,
    programming_pin: &'a mut Pin,

    // zero-sized markers
    _mode: PhantomData<Mode>,
    _speed: PhantomData<Speed>,

    channel: Channel,
    power: Power,
}

impl<'a, Device, Pin> HC12<'a, Device, Pin, Fu3, B9600>
where
    Device: Read + Write,
    Pin: OutputPin,
{
    /// Create a new builder in programming mode. The serial port
    /// MUST be set to 9600 BPS to be able to communicate with the
    /// on-board microcontroller, in order to program properly.
    ///
    /// For most HALs, this is an infallible operation, as setting a pin
    /// is a default item.
    ///
    /// This function will block for not less than 40ms.
    pub fn factor_settings(
        device: &'a mut Device,
        programming_pin: &'a mut Pin,
        delay: &mut impl DelayNs,
    ) -> Result<Self, Error<Pin::Error>> {
        // enter AT (programming) mode
        programming_pin.set_low().map_err(Error::DeviceError)?;
        delay.delay_ms(40);

        Ok(HC12 {
            device,
            programming_pin,
            _mode: PhantomData,
            _speed: PhantomData,
            channel: Channel::default(),
            power: Power::default(),
        })
    }
}

impl<'a, Device, Pin, Mode, Speed> HC12<'a, Device, Pin, Mode, Speed> {
    /// Set the power of the module. The default power is the maxumum
    /// of P8
    pub fn power(mut self, power: Power) -> Self {
        self.power = power;
        self
    }

    /// Set the channel. The module by default is set to Channel 0
    pub fn channel(mut self, channel: Channel) -> Self {
        self.channel = channel;
        self
    }

    /// Program into Fu1 mode.
    ///
    /// Fu1 is a moderate power-saving mode, with an idle current of ~3.5mA.
    /// Fu1 supports all speeds, but the in-air baudrate remains 250000 bps
    pub fn fu1(self) -> HC12<'a, Device, Pin, Fu1, Speed>
    where
        Speed: ValidSpeed,
        Fu1: ValidModeFor<Speed> + Default,
    {
        HC12 {
            device: self.device,
            programming_pin: self.programming_pin,
            _mode: PhantomData,
            _speed: self._speed,
            channel: self.channel,
            power: self.power,
        }
    }

    /// Fu2 is the extreme power-saving mode of the HC-12. This mode only
    /// supports B1200, B2400, and B4800 only. The in-air baudrate is a uniform 250000 bps.
    /// It is reccomended to send packets over this mode at a frequency not exceeding 1Hz.
    pub fn fu2(self) -> HC12<'a, Device, Pin, Fu2, Speed>
    where
        Speed: ValidSpeed,
        Fu3: ValidModeFor<Speed> + Default,
    {
        HC12 {
            device: self.device,
            programming_pin: self.programming_pin,
            _mode: PhantomData,
            _speed: self._speed,
            channel: self.channel,
            power: self.power,
        }
    }

    /// Fu3 is the premier full-speed mode of the radio module. It accepts any speed, and will
    /// adjust the in-air speed to the speed of the local serial speed. The higher the speed
    /// the lower the sensitivity, and thus, the range. This is the default factory  mode of the
    /// device.
    pub fn fu3(self) -> HC12<'a, Device, Pin, Fu3, Speed>
    where
        Speed: ValidSpeed,
        Fu3: ValidModeFor<Speed> + Default,
    {
        HC12 {
            device: self.device,
            programming_pin: self.programming_pin,
            _mode: PhantomData,
            _speed: self._speed,
            channel: self.channel,
            power: self.power,
        }
    }

    /// Fu4 mode is the long range mode of the device, and can achive communication distances of up
    /// to 1.8km. Only 1200 bps is supported. In the air the baud rate will be redueced to a
    /// whopping 500bps.
    ///
    /// Usage notes:
    /// - Avoid transmitting more than 60 bytes in a packet
    /// - Transmit a packet not more than once every two seconds.
    pub fn fu4(self) -> HC12<'a, Device, Pin, Fu4, Speed>
    where
        Speed: ValidSpeed,
        Fu4: ValidModeFor<Speed> + Default,
    {
        HC12 {
            device: self.device,
            programming_pin: self.programming_pin,
            _mode: PhantomData,
            _speed: self._speed,
            channel: self.channel,
            power: self.power,
        }
    }

    /// Program into 1200 bps.
    pub fn b1200(self) -> HC12<'a, Device, Pin, Mode, B1200>
    where
        Mode: ValidModeFor<B1200> + Default,
        B1200: ValidSpeed + Default,
    {
        HC12 {
            device: self.device,
            programming_pin: self.programming_pin,
            _mode: self._mode,
            _speed: PhantomData,
            channel: self.channel,
            power: self.power,
        }
    }

    /// Program into 2400 bps.
    pub fn b2400(self) -> HC12<'a, Device, Pin, Mode, B2400>
    where
        Mode: ValidModeFor<B2400>,
    {
        HC12 {
            device: self.device,
            programming_pin: self.programming_pin,
            _mode: self._mode,
            _speed: PhantomData,
            channel: self.channel,
            power: self.power,
        }
    }

    /// Program into 4800 bps.
    pub fn b4800(self) -> HC12<'a, Device, Pin, Mode, B4800>
    where
        Mode: ValidModeFor<B4800>,
    {
        HC12 {
            device: self.device,
            programming_pin: self.programming_pin,
            _mode: self._mode,
            _speed: PhantomData,
            channel: self.channel,
            power: self.power,
        }
    }

    /// Program into 9600 bps.
    pub fn b9600(self) -> HC12<'a, Device, Pin, Mode, B9600>
    where
        Mode: ValidModeFor<B9600>,
    {
        HC12 {
            device: self.device,
            programming_pin: self.programming_pin,
            _mode: self._mode,
            _speed: PhantomData,
            channel: self.channel,
            power: self.power,
        }
    }

    /// Program into 19200 bps.
    pub fn b19200(self) -> HC12<'a, Device, Pin, Mode, B19200>
    where
        Mode: ValidModeFor<B19200>,
    {
        HC12 {
            device: self.device,
            programming_pin: self.programming_pin,
            _mode: self._mode,
            _speed: PhantomData,
            channel: self.channel,
            power: self.power,
        }
    }

    /// Program into 39400 bps.
    pub fn b39400(self) -> HC12<'a, Device, Pin, Mode, B39400>
    where
        Mode: ValidModeFor<B39400>,
    {
        HC12 {
            device: self.device,
            programming_pin: self.programming_pin,
            _mode: self._mode,
            _speed: PhantomData,
            channel: self.channel,
            power: self.power,
        }
    }

    /// Program into 57600 bps.
    pub fn b57600(self) -> HC12<'a, Device, Pin, Mode, B57600>
    where
        Mode: ValidModeFor<B57600>,
    {
        HC12 {
            device: self.device,
            programming_pin: self.programming_pin,
            _mode: self._mode,
            _speed: PhantomData,
            channel: self.channel,
            power: self.power,
        }
    }

    /// Program into 115200 bps.
    pub fn b115200(self) -> HC12<'a, Device, Pin, Mode, B115200>
    where
        Mode: ValidModeFor<B115200>,
    {
        HC12 {
            device: self.device,
            programming_pin: self.programming_pin,
            _mode: self._mode,
            _speed: PhantomData,
            channel: self.channel,
            power: self.power,
        }
    }
}

impl<'a, Device, Pin, Mode, Speed> HC12<'a, Device, Pin, Mode, Speed>
where
    Device: Read + Write + ReadReady,
    Pin: OutputPin,
    Mode: ValidMode + ValidModeFor<Speed>,
    Speed: ValidSpeed,
{
    /// Program the HC12
    pub fn program(&mut self, delay: &mut impl DelayNs) -> Result<(), Error<Device::Error>> {
        run_command(self.device, Speed::default(), delay)?;
        run_command(self.device, Mode::default(), delay)?;
        run_command(self.device, self.power, delay)?;
        run_command(self.device, self.channel, delay)
    }

    /// Return the HC-12 to transparent mode. For most HALs, this is
    /// infallible, as it only relies on setting a pin high or low.
    /// This function will block for not less than 80ms.
    pub fn into_transparent_mode(
        self,
        delay: &mut impl DelayNs,
    ) -> Result<TransparentHC12<'a, Device, Pin, Mode, Speed>, Pin::Error> {
        self.programming_pin.set_high()?;
        delay.delay_ms(80);

        Ok(TransparentHC12::new(
            self.device,
            self.programming_pin,
            self.channel,
            self.power,
        ))
    }
}

/// A transparent HC-12 device. This can be used directly as a serial device,
/// or returned to AT (programming) mode, or decomposed to return the pin and the
/// serial device used in programming the module
pub struct TransparentHC12<'a, Device, Pin, Mode, Speed> {
    device: &'a mut Device,
    pin: &'a mut Pin,
    mode: PhantomData<Mode>,
    speed: PhantomData<Speed>,
    channel: Channel,
    power: Power,
}

impl<'a, Device, Pin, Mode, Speed> TransparentHC12<'a, Device, Pin, Mode, Speed>
where
    Device: ErrorType,
    Pin: OutputPin,
{
    pub(crate) fn new(
        device: &'a mut Device,
        pin: &'a mut Pin,
        channel: Channel,
        power: Power,
    ) -> Self {
        Self {
            device,
            pin,
            channel,
            power,
            speed: PhantomData,
            mode: PhantomData,
        }
    }

    /// Get the current programmed channel
    pub fn channel(&self) -> &Channel {
        &self.channel
    }

    /// The current programmed power
    pub fn power(&self) -> &Power {
        &self.power
    }

    /// Decompose the device to its serial port and programming pin
    pub fn inner(self) -> (&'a mut Device, &'a mut Pin) {
        (self.device, self.pin)
    }

    /// Return to programming mode. This persists the programming parameters from the last
    /// probramming of the device. In most HALs this is infallible.
    pub fn into_programming_mode(
        self,
        delay: &mut impl DelayNs,
    ) -> Result<HC12<'a, Device, Pin, Mode, Speed>, Error<Pin::Error>>
    where
        Device: Read + Write,
        Pin: OutputPin,
    {
        self.pin.set_low().map_err(Error::DeviceError)?;
        delay.delay_ms(40);

        Ok(HC12 {
            device: self.device,
            programming_pin: self.pin,
            _mode: PhantomData,
            _speed: PhantomData,
            channel: self.channel,
            power: self.power,
        })
    }
}

impl<'a, Device, Pin, Mode, Speed> ErrorType for TransparentHC12<'a, Device, Pin, Mode, Speed>
where
    Device: ErrorType,
{
    type Error = Device::Error;
}

impl<'a, Device, Pin, Mode, Speed> Read for TransparentHC12<'a, Device, Pin, Mode, Speed>
where
    Device: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.device.read(buf)
    }
}

impl<'a, Device, Pin, Mode, Speed> ReadReady for TransparentHC12<'a, Device, Pin, Mode, Speed>
where
    Device: ReadReady,
{
    fn read_ready(&mut self) -> Result<bool, Self::Error> {
        self.device.read_ready()
    }
}

impl<'a, Device, Pin, Mode, Speed> Write for TransparentHC12<'a, Device, Pin, Mode, Speed>
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

impl<'a, Device, Pin, Mode, Speed> WriteReady for TransparentHC12<'a, Device, Pin, Mode, Speed>
where
    Device: WriteReady,
{
    fn write_ready(&mut self) -> Result<bool, Self::Error> {
        self.device.write_ready()
    }
}
