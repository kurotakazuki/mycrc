#![no_std]
#![doc = include_str!("../README.md")]

pub use self::algorithm::{Algorithm, CHECK_BYTES};
pub use self::crc::CRC;

/// CRC algorithm
mod algorithm;
/// Cyclic redundancy check
mod crc;
