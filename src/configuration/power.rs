/// Power ranges from 1-8 on the HC-12, with 1 being the lowest power and 8 being the highest
/// These translate to dBm from -1 to 20
#[derive(Debug, Clone, Copy, PartialEq, Eq, defmt::Format)]
pub enum Power {
    /// -1 dBm
    P1,
    /// 2 dBm
    P2,
    /// 5 dBm
    P3,
    /// 8 dBm
    P4,
    /// 11 dBm
    P5,
    /// 14 dBm
    P6,
    /// 17 dBm
    P7,
    /// 20 dBm
    P8,
}

impl Power {
    /// Returns the dBm value of the power level
    #[allow(non_snake_case)]
    pub const fn dBm(&self) -> i8 {
        match self {
            Power::P1 => -1,
            Power::P2 => 2,
            Power::P3 => 5,
            Power::P4 => 8,
            Power::P5 => 11,
            Power::P6 => 14,
            Power::P7 => 17,
            Power::P8 => 20,
        }
    }
}

impl Default for Power {
    /// The default power level at factory programming is 8
    fn default() -> Self {
        Power::P8
    }
}
