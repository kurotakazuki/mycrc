#![doc = include_str!("../README.md")]
#![no_std]

pub use self::algorithm::{Algorithm, Endian};
pub use self::crc::CRC;

/// CRC algorithm
mod algorithm;
/// Cyclic redundancy check
mod crc;
