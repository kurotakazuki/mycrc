use core::mem;

/// CRC algorithm.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Algorithm<T> {
    pub poly: T,
    pub init: T,
    pub refin: bool,
    pub refout: bool,
    pub xorout: T,
    pub residue: T,
}

macro_rules! algorithm_impl {
    ( $( $t:ty ),* ) => ($(
        impl Algorithm<$t> {
            /// Initialize value.
            pub const fn initialize(init: $t, refin: bool) -> $t {
                if refin {
                    init.reverse_bits()
                } else {
                    init
                }
            }

            pub(crate) const fn optional_reflection(refin: bool, refout: bool, value: $t) -> $t {
                if refin ^ refout {
                    value.reverse_bits()
                } else {
                    value
                }
            }

            /// Finalize value.
            /// Change value to checksum.
            pub const fn finalize(refin: bool, refout: bool, xorout: $t, value: $t) -> $t {
                Self::optional_reflection(refin, refout, value) ^ xorout
            }

            /// Caluculate byte with reciprocal polynomial.
            pub const fn calc_byte_with_reciprocal_poly(reciprocal_poly: $t, refin: bool, byte: u8) -> $t {
                let mut value = if refin {
                    byte as $t
                } else {
                    byte.reverse_bits() as $t
                };

                let mut i = 0;
                while i < 8 {
                    if value & 1 == 0 {
                        value >>= 1;
                    } else {
                        value >>= 1;
                        value ^= reciprocal_poly;
                    }
                    i += 1;
                }

                if refin {
                    value
                } else {
                    value.reverse_bits()
                }
            }

            /// Create table.
            pub const fn create_table(poly: $t, refin: bool) -> [$t; 256] {
                let mut table = [0; 256];
                let reciprocal_poly = poly.reverse_bits();

                let mut i = 0;
                while i < table.len() {
                    table[i] = Self::calc_byte_with_reciprocal_poly(reciprocal_poly, refin, i as u8);
                    i += 1;
                }

                table
            }

            /// Caluculate bytes with values.
            pub const fn calc_bytes_with_values(refin: bool, mut value: $t, bytes: &[u8], table: &[$t; 256]) -> $t {
                let mut i = 0;
                if refin {
                    while i < bytes.len() {
                        value = table[(value as usize ^ bytes[i] as usize) & 0xFF] ^ (value >> 8);
                        i += 1;
                    }
                } else {
                    while i < bytes.len() {
                        value = table[((value >> (mem::size_of::<$t>() * 8 - 8)) as usize ^ bytes[i] as usize) & 0xFF] ^ (value << 8);
                        i += 1;
                    }
                }
                value
            }
        }
    )*)
}

algorithm_impl!(u16, u32, u64, u128);
