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

// Feature impls
#[cfg(feature = "i128")]
impl_reverse!(i128);
#[cfg(feature = "u128")]
impl_reverse!(u128);
#[cfg(feature = "isize")]
impl_reverse!(isize);
#[cfg(feature = "usize")]
impl_reverse!(usize);