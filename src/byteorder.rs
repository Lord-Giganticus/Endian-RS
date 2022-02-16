use std::fmt::*;


#[derive(Copy, Clone, PartialEq)]
pub enum ByteOrder {
    Little,
    Big
}

impl Display for ByteOrder {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        return match *self {
            ByteOrder::Little => write!(f, "Little"),
            ByteOrder::Big => write!(f, "Big")
        }
    }
}

impl Default for ByteOrder {
    fn default() -> ByteOrder {
        if cfg!(target_endian = "big") {
            return ByteOrder::Big;
        } else {
            return ByteOrder::Little;
        }
    }
}

impl ByteOrder {
    pub fn new() -> Self {
        return ByteOrder::default();
    }
}