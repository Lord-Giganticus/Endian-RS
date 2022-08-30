use std::fmt::Display;

#[cfg(feature = "binread")]
use binread;

#[cfg(feature = "binwrite")]
use binwrite;

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

#[cfg(feature = "binread")]
impl Into<binread::Endian> for Endian {
    fn into(self) -> binread::Endian {
        match self {
            Endian::Big => binread::endian::BE,
            Endian::Little => binread::endian::LE
        }
    }
}

#[cfg(feature = "binwrite")]
impl Into<binwrite::Endian> for Endian {
    fn into(self) -> binwrite::Endian {
        match self {
            Endian::Big => binwrite::Endian::Big,
            Endian::Little => binwrite::Endian::Little
        }
    }
}