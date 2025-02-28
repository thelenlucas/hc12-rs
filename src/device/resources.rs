/// The HC-12 device depends on a few resouces to function. These resources are the UART, the set pin that controlls the mode
/// of the device, and the clock that lets the host delay for the device to respond, as the AT mode is rather slow.
/// It's possible to use the device without a pin and delay, but the device may not be programmed, and the user is responsible
/// for ensuring that the device is properly configured before use.
pub struct HC12ProgrammingResources<PIN, DELAY> {
    /// The pin that is used to control the mode of the device
    pub(super) mode_pin: PIN,
    /// The delay that is used to wait for the device to respond
    pub(super) delay: DELAY,
}
impl<PIN, DELAY> HC12ProgrammingResources<PIN, DELAY> {
    /// Creates a new HC12ProgrammingResources with the given pin and delay
    pub const fn new(mode_pin: PIN, delay: DELAY) -> Self {
        HC12ProgrammingResources { mode_pin, delay }
    }

    /// Releases the underlying resources
    pub fn release(self) -> (PIN, DELAY) {
        (self.mode_pin, self.delay)
    }
}

/// No programming resources are used for the device, and the user is responsible for ensuring that the device
/// is properly configured before use.
pub struct NonConfigurable;
impl crate::sealed::Sealed for NonConfigurable {} // Prevents the user from using this struct directly
