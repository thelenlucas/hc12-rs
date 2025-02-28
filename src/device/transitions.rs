use core::fmt::Debug;

use embedded_hal::{
    delay::DelayNs,
    digital::{ErrorType, OutputPin},
};

use crate::{
    configuration::{baudrates::B9600, Baudrate},
    modes::{Stolen, ValidHC12Mode, AT, FU3},
    sealed::Sealed,
};

use super::{ValidProgrammingResources, HC12};

/// A failed transition returns the reason it failed, plus the UART and pin, so we get the
/// underlying devices back
pub struct HC12Error<E: Debug, U, P> {
    pub error: E,
    pub uart: U,
    pub pin: P,
}
impl<E: Debug, U, P> Debug for HC12Error<E, U, P> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "HC12Error {{ error: {:?}}}", self.error)
    }
}

/// Trait for transitioning into AT mode. To move into AT mode, the current baudrate must be 9600
pub trait IntoATMode<U, P, O: Baudrate>
where
    P: ErrorType + ValidProgrammingResources + DelayNs,
{
    /// Transition into AT mode. This fails if the pin fails to pull low
    fn into_at_mode(self) -> Result<HC12<U, P, AT<O>, B9600>, HC12Error<P::Error, U, P>>;
}

/// Re-implimente ErrorType on OutputPin-containing HC-12s
impl<U, P, M, B> ErrorType for HC12<U, P, M, B>
where
    P: ErrorType,
{
    type Error = P::Error;
}

/// Allow stolen to transition to AT mode, if the current baudrate is 9600. The baudrate inside AT<O> may be different
/// but the baudrate of the underlying device must be 9600. Pass O to AT, so it is aware of the programmed baudrate
impl<U, P: ValidProgrammingResources + OutputPin + DelayNs, O: Baudrate> IntoATMode<U, P, O>
    for HC12<U, P, Stolen<FU3<O>, O>, B9600>
{
    fn into_at_mode(self) -> Result<HC12<U, P, AT<O>, B9600>, HC12Error<P::Error, U, P>> {
        let inner = self.into_inner();
        let uart = inner.0;
        let mut pin = inner.1;
        let mode = inner.2;
        let configuration = mode.get_config();
        match pin.set_low() {
            Ok(()) => {
                // We can transition to AT mode
                Ok(HC12::new(
                    uart,
                    pin,
                    AT::new(mode.get_old_mode().get_baudrate(), configuration),
                    B9600,
                ))
            }
            Err(e) => {
                // We failed to transition to AT mode
                Err(HC12Error {
                    error: e,
                    uart,
                    pin,
                })
            }
        }
    }
}

/// Allow FU3 to transition to AT mode under the same conditions as stolen
impl<U, P: ValidProgrammingResources + OutputPin + DelayNs, O: Baudrate> IntoATMode<U, P, O>
    for HC12<U, P, FU3<O>, B9600>
{
    fn into_at_mode(self) -> Result<HC12<U, P, AT<O>, B9600>, HC12Error<P::Error, U, P>> {
        let inner = self.into_inner();
        let uart = inner.0;
        let mut programming = inner.1;
        let mode = inner.2;
        let configuration = mode.get_config();
        match programming.set_low() {
            Ok(()) => {
                // We can transition to AT mode.
                programming.delay_ms(100); // Delay as per the datasheet
                Ok(HC12::new(
                    uart,
                    programming,
                    AT::new(mode.get_baudrate(), configuration),
                    B9600,
                ))
            }
            Err(e) => {
                // We failed to transition to AT mode
                Err(HC12Error {
                    error: e,
                    uart,
                    pin: programming,
                })
            }
        }
    }
}

/// Trait for transitioning into FU3 mode. Any baudrate can transition into FU3, but the current mode must be AT, and the
/// programmed baudrate must be maintained inside FU3. ProgrammingResources isn't needed here, because we're moving into
/// a non-programming mode
pub trait IntoFU3Mode<U, P, O: Baudrate, D: Baudrate>: Sealed + ErrorType {
    /// Transition into FU3 mode. This fails if the pin fails to pull high
    fn into_fu3_mode(self) -> Result<HC12<U, P, FU3<O>, D>, HC12Error<Self::Error, U, P>>;
}

/// Allow AT to transition to FU3 mode. This case is trivial, because AT mode is always in 9600 baudrate, so the underlying
/// baudrate will alwasy be 9600, even if the programmbed baudrate inside FU3<> is different. Because FU3 allows for any baudrate,
/// this is a valid transition
impl<U, P: OutputPin, O: Baudrate> IntoFU3Mode<U, P, O, B9600> for HC12<U, P, AT<O>, B9600> {
    fn into_fu3_mode(self) -> Result<HC12<U, P, FU3<O>, B9600>, HC12Error<Self::Error, U, P>> {
        let inner = self.into_inner();
        let uart = inner.0;
        let mut pin = inner.1;
        let mode = inner.2;
        let configuration = mode.get_config();
        match pin.set_high() {
            Ok(()) => {
                // We can transition to FU3 mode
                Ok(HC12::new(
                    uart,
                    pin,
                    FU3::new(mode.current_programmed_baudrate, configuration),
                    B9600,
                ))
            }
            Err(e) => {
                // We failed to transition to FU3 mode
                Err(HC12Error {
                    error: e,
                    uart,
                    pin,
                })
            }
        }
    }
}
