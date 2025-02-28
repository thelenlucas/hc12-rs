use core::fmt::Debug;

use defmt::Format;

use crate::sealed::Sealed;

/// A valid baudrate
pub trait ValidBaudrate: Sealed + Copy + Clone + Debug + Format + Default {
    const BAUDRATE: u32;
}

#[derive(Debug, Clone, Copy, Format, Default)]
pub struct B1200;
impl Sealed for B1200 {}
impl ValidBaudrate for B1200 {
    const BAUDRATE: u32 = 1200;
}

#[derive(Debug, Clone, Copy, Format, Default)]
pub struct B2400;
impl Sealed for B2400 {}
impl ValidBaudrate for B2400 {
    const BAUDRATE: u32 = 2400;
}

#[derive(Debug, Clone, Copy, Format, Default)]
pub struct B4800;
impl Sealed for B4800 {}
impl ValidBaudrate for B4800 {
    const BAUDRATE: u32 = 4800;
}

#[derive(Debug, Clone, Copy, Format, Default)]
pub struct B9600;
impl Sealed for B9600 {}
impl ValidBaudrate for B9600 {
    const BAUDRATE: u32 = 9600;
}

#[derive(Debug, Clone, Copy, Format, Default)]
pub struct B19200;
impl Sealed for B19200 {}
impl ValidBaudrate for B19200 {
    const BAUDRATE: u32 = 19200;
}

#[derive(Debug, Clone, Copy, Format, Default)]
pub struct B38400;
impl Sealed for B38400 {}
impl ValidBaudrate for B38400 {
    const BAUDRATE: u32 = 38400;
}

#[derive(Debug, Clone, Copy, Format, Default)]
pub struct B57600;
impl Sealed for B57600 {}
impl ValidBaudrate for B57600 {
    const BAUDRATE: u32 = 57600;
}

#[derive(Debug, Clone, Copy, Format, Default)]
pub struct B115200;
impl Sealed for B115200 {}
impl ValidBaudrate for B115200 {
    const BAUDRATE: u32 = 115200;
}
