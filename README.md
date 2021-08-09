# My CRC

[![Crate](https://img.shields.io/crates/v/mycrc.svg)](https://crates.io/crates/mycrc)
[![API](https://docs.rs/mycrc/badge.svg)](https://docs.rs/mycrc)

Create your own cyclic redundancy check (CRC).

## Getting Started
1. Create your own CRC using [`CRC::new`].
2. Create checksum from message.
    - [`CRC::checksum`]
    - [`CRC::initialize`] -> [`CRC::calc_bytes`] -> ... -> [`CRC::calc_bytes`] -> [`CRC::finalize`]
3. Use [`CRC::is_error_free`] to check if bytes [message + checksum] are error-free.

### Example
```rust
use mycrc::{Algorithm, CRC, Endian};

// message
const CHECK_BYTES: &[u8] = b"123456789";

// Create your own CRC.
let mut crc32c = CRC::<u32>::new(
    Endian::Little, // endian
    0x1edc6f41, // poly
    0xffffffff, // init
    true, // refin
    true, // refout
    0xffffffff, // xorout
);

// Checksum
assert_eq!(crc32c.checksum(CHECK_BYTES), 0xe3069283);
// Is error free?
let checksum = crc32c.checksum_to_endian_bytes(CHECK_BYTES);
let bytes = [CHECK_BYTES, &checksum].concat();
assert!(crc32c.is_error_free(&bytes));
```
