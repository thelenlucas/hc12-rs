use crate::{configuration::HC12Configuration, sealed};

mod at;
mod stolen;
mod transparent;

/// A valid HC-12 Mode
pub trait ValidHC12Mode: sealed::Sealed + Copy {
    fn get_config(&self) -> HC12Configuration;
}

pub use at::*;
pub use stolen::*;
pub use transparent::*;
