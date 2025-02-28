use crate::configuration::{Baudrate, HC12Configuration};

use super::ValidHC12Mode;

/// There are three transparent modes
pub trait ValidTransparentMode: crate::sealed::Sealed + ValidHC12Mode {
    fn transmission_time_delay(&self) -> u32;
}

/// FU3 is the default full-speed transparent mode of the HC-12
#[derive(Copy, Clone)]
pub struct FU3<B: Baudrate> {
    baudrate: B,
    current_configuration: HC12Configuration,
}
impl<B: Baudrate> crate::sealed::Sealed for FU3<B> {}
impl<B: Baudrate> ValidHC12Mode for FU3<B> {
    fn get_config(&self) -> HC12Configuration {
        self.current_configuration
    }
}
impl<B: Baudrate> ValidTransparentMode for FU3<B> {
    fn transmission_time_delay(&self) -> u32 {
        0
    }
}

/// FU3 is the default mode at programming time, this is a quick entry point

impl<B: Baudrate> FU3<B> {
    pub fn new(baudrate: B, configuration: HC12Configuration) -> Self {
        FU3 {
            baudrate,
            current_configuration: configuration,
        }
    }

    pub fn get_baudrate(&self) -> B {
        self.baudrate
    }

    pub fn get_configuration(&self) -> HC12Configuration {
        self.current_configuration
    }
}
