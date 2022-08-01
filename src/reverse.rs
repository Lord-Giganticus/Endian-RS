pub trait Reverse {
    fn reverse(&self) -> Self;
}

macro_rules! impl_reverse {
    ($arg:ty) => {
        impl Reverse for $arg {
            fn reverse(&self) -> Self {
                let mut buf = self.to_ne_bytes();
                buf.reverse();
                Self::from_ne_bytes(buf)
            }
        }
    };
    ($arg:ty, $($args:ty),+) => {
        impl_reverse!($arg);
        impl_reverse!($($args),+);
    };
}

impl_reverse!(u8, i8, u16, i16, u32, i32, u64, i64);