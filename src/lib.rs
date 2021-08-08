#![no_std]
#![doc = include_str!("../README.md")]

pub use self::algorithm::Algorithm;
pub use self::crc::CRC;

/// CRC algorithm
mod algorithm;
/// Cyclic redundancy check
mod crc;
