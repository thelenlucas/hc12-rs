use core::fmt::Write;
use heapless::String;

use crate::commands::Command;

pub trait ValidSpeed: Default {
    /// Speed in bits per second
    fn bps() -> u32;
}

#[derive(Debug, Default)]
pub struct B1200 {}

#[derive(Debug, Default)]
pub struct B2400 {}

#[derive(Debug, Default)]
pub struct B4800 {}

#[derive(Debug, Default)]
pub struct B9600 {}

#[derive(Debug, Default)]
pub struct B19200 {}

#[derive(Debug, Default)]
pub struct B39400 {}

#[derive(Debug, Default)]
pub struct B57600 {}

#[derive(Debug, Default)]
pub struct B115200 {}

impl ValidSpeed for B1200 {
    fn bps() -> u32 {
        1200
    }
}

impl ValidSpeed for B2400 {
    fn bps() -> u32 {
        2400
    }
}

impl ValidSpeed for B4800 {
    fn bps() -> u32 {
        4800
    }
}

impl ValidSpeed for B9600 {
    fn bps() -> u32 {
        9600
    }
}

impl ValidSpeed for B19200 {
    fn bps() -> u32 {
        19200
    }
}

impl ValidSpeed for B39400 {
    fn bps() -> u32 {
        39400
    }
}

impl ValidSpeed for B57600 {
    fn bps() -> u32 {
        57600
    }
}

impl ValidSpeed for B115200 {
    fn bps() -> u32 {
        115200
    }
}

impl<T> Command for T
where
    T: ValidSpeed,
{
    fn command(&self) -> heapless::String<16> {
        let mut s = String::new();

        write!(&mut s, "AT+B{}", T::bps()).ok();

        s
    }
}
