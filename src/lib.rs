//! Small derive macro to safely convert the numerical value of an enum to the enum.

#![no_std]

pub use asenum_derive::{self as derive, Convert};

/// Convert allows safe, checked conversion between the numeric representation
/// of the enum and the enum.
pub trait Convert: Sized {
    /// Converts the numeric representation to its enum value.
    ///
    /// Returns `None` if the option is not an enum value.
    fn from_value(value: usize) -> Option<Self>;
}
