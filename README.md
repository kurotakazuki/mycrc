# My CRC

[![Crate](https://img.shields.io/crates/v/mycrc.svg)](https://crates.io/crates/mycrc)
[![API](https://docs.rs/mycrc/badge.svg)](https://docs.rs/mycrc)

Create your own cyclic redundancy check (CRC).

## Getting Started
1. Create your own CRC [`Algorithm`].
2. [`CRC`]
    - [`CRC::checksum`]
    - [`CRC::initialize`] -> [`CRC::calc_bytes`] -> [`CRC::finalize`]


### Example
```rust
use mycrc::{Algorithm, CHECK_BYTES, CRC};

let algo_iscsi = Algorithm::<u32> {
    poly: 0x1edc6f41,
    init: 0xffffffff,
    refin: true,
    refout: true,
    xorout: 0xffffffff,
    check: 0xe3069283,
    residue: 0xb798b438,
};
let mut crc32c = CRC::<u32>::new(algo_iscsi);

assert_eq!(crc32c.checksum(CHECK_BYTES), crc32c.algorithm.check);
```
