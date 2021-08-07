use core::mem;

#[derive(Copy, Clone, Debug)]
pub struct Algorithm<T> {
    pub poly: T,
    pub init: T,
    pub refin: bool,
    pub refout: bool,
    pub xorout: T,
    pub check: T,
    pub residue: T,
}

macro_rules! algorithm_impl {
    ( $( $t:ty ),* ) => ($(
        impl Algorithm<$t> {
            pub const fn initialize(&self) -> $t {
                if self.refin {
                    self.init.reverse_bits()
                } else {
                    self.init
                }
            }

            pub const fn finalize(&self, value: $t) -> $t {
                let value = if self.refin ^ self.refout {
                    value.reverse_bits()
                } else {
                    value
                };

                value ^ self.xorout
            }

            pub const fn calc_byte(reciprocal_poly: $t, refin: bool, byte: u8) -> $t {
                let mut c = if refin {
                    byte as $t
                } else {
                    byte.reverse_bits() as $t
                };

                let mut i = 0;
                while i < 8 {
                    if c & 1 == 0 {
                        c >>= 1;
                    } else {
                        c >>= 1;
                        c ^= reciprocal_poly;
                    }
                    i += 1;
                }

                if refin {
                    c
                } else {
                    c.reverse_bits()
                }
            }

            pub const fn create_table_with_reciprocal_poly(&self) -> [$t; 256] {
                let mut table = [0; 256];
                let reciprocal_poly = self.poly.reverse_bits();

                let mut i = 0;
                while i < table.len() {
                    table[i] = Self::calc_byte(reciprocal_poly, self.refin, i as u8);
                    i += 1;
                }

                table
            }

            pub const fn calc_bytes_with_values(&self, mut value: $t, bytes: &[u8], table: &[$t; 256]) -> $t {
                let mut i = 0;
                if self.refin {
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

// TODO impl u8
algorithm_impl!(u16, u32, u64, u128);
