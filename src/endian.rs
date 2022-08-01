use std::fmt::Display;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Endian {
    Big,
    Little
}

impl Default for Endian {
    fn default() -> Self {
        NATIVE
    }
}

impl Display for Endian {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Endian::*;
        match *self {
            Big => write!(f, "Big"),
            Little => write!(f, "Little")
        }
    }
}

#[cfg(target_endian="little")]
pub const NATIVE: Endian = Endian::Little;

#[cfg(target_endian="big")]
pub const NATIVE: Endian = Endian::Big;