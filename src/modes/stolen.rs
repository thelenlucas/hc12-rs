use crate::{
    configuration::{Baudrate, HC12Configuration},
    HC12,
};

use super::{ValidHC12Mode, FU3};

/// Represents a stolen UART
pub struct StolenUart;

/// Stolen mode is an concession that we have to make due to the way that embedded-hal and embedded-io work.
/// Because these crates don't provide a trait for underlying access, modification, and verification of baudrates,
/// the user application must be provided a way to manage them, if they want to change them.
#[derive(Copy, Clone, Debug, defmt::Format)]
pub struct Stolen<StolenMode: ValidHC12Mode, CurrentProgrammedBaudRate: Baudrate> {
    old_mode: StolenMode,
    current_programmed_baudrate: CurrentProgrammedBaudRate,
    current_configuration: HC12Configuration,
}
impl<S: ValidHC12Mode, C: Baudrate> crate::sealed::Sealed for Stolen<S, C> {}
impl<S: ValidHC12Mode, C: Baudrate> ValidHC12Mode for Stolen<S, C> {
    fn get_config(&self) -> HC12Configuration {
        self.current_configuration
    }
}
impl<S: ValidHC12Mode, C: Baudrate> Stolen<S, C> {
    pub(crate) fn new(
        old_mode: S,
        current_programmed_baudrate: C,
        current_configuration: HC12Configuration,
    ) -> Self {
        Stolen {
            old_mode,
            current_programmed_baudrate,
            current_configuration,
        }
    }

    pub fn get_old_mode(&self) -> S {
        self.old_mode
    }
}

/// This trait allows an HC12 to have it's uart stolen from it
pub trait StealUart<U, P, M: ValidHC12Mode, B: Baudrate> {
    /// Steal the Uart from the HC12, returning the new HC12 with the stolen mode, and the UART
    fn steal_uart(self) -> (HC12<StolenUart, P, Stolen<M, B>, B>, U);
}

/// Similar, but this allows the user to return a (potentially new) baudrate and uart
pub trait ReturnUart<Uart, P, M: ValidHC12Mode, B: Baudrate> {
    /// Return the Uart to the HC12, returning the new HC12 with the stolen mode, and the UART
    fn return_uart<N: Baudrate>(self, uart: Uart, new_baudrate: N) -> HC12<Uart, P, M, N>;
}

/// Steal from FU3
impl<U, P, B: Baudrate> StealUart<U, P, FU3<B>, B> for HC12<U, P, FU3<B>, B> {
    fn steal_uart(self) -> (HC12<StolenUart, P, Stolen<FU3<B>, B>, B>, U) {
        let inner = self.into_inner();
        let old_uart = inner.0;
        let old_pin = inner.1;
        let old_mode = inner.2;
        let old_baud = inner.3;
        let old_config = old_mode.get_config();

        (
            HC12::new(
                StolenUart,
                old_pin,
                Stolen::new(old_mode, old_baud, old_config),
                old_baud,
            ),
            old_uart,
        )
    }
}

/// Any baudrate can be returned to FU3, provided of course, that the old mode was FU3
/// However, the FU3<B> remains as the old mode, which allows AT mode to verify baudrates
impl<U, P, B: Baudrate> ReturnUart<U, P, FU3<B>, B> for HC12<StolenUart, P, Stolen<FU3<B>, B>, B> {
    fn return_uart<N: Baudrate>(self, uart: U, new_baudrate: N) -> HC12<U, P, FU3<B>, N> {
        let inner = self.into_inner();
        let old_pin = inner.1;
        let old_mode = inner.2;
        let old_config = old_mode.get_config();

        HC12::new(
            uart,
            old_pin,
            FU3::new(old_mode.current_programmed_baudrate, old_config),
            new_baudrate,
        )
    }
}

/// Steal from AT. This can always be done
impl<U, P, B: Baudrate> StealUart<U, P, super::AT<B>, B> for HC12<U, P, super::AT<B>, B> {
    fn steal_uart(self) -> (HC12<StolenUart, P, Stolen<super::AT<B>, B>, B>, U) {
        let inner = self.into_inner();
        let old_uart = inner.0;
        let old_pin = inner.1;
        let old_mode = inner.2;
        let old_baud = inner.3;
        let old_config = old_mode.get_config();

        (
            HC12::new(
                StolenUart,
                old_pin,
                Stolen::new(old_mode, old_baud, old_config),
                old_baud,
            ),
            old_uart,
        )
    }
}
