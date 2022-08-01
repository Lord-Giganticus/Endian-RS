use crate::reverse::Reverse;

pub trait FromBytes {
    fn from_bytes(buf: &[u8]) -> Self;
}

pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}

pub trait Numeric : Reverse + FromBytes + ToBytes + Copy {}

impl<T: Reverse + FromBytes + ToBytes + Copy> Numeric for T {}

macro_rules! impl_from_bytes {
    ($arg:ty) => {
        impl FromBytes for $arg {
            fn from_bytes(buf: &[u8]) -> Self {
                Self::from_ne_bytes(buf.try_into().unwrap())
            }
        }
    };
    ($arg:ty, $($args:ty),+) => {
        impl_from_bytes!($arg);
        impl_from_bytes!($($args),+);
    };
}

macro_rules! impl_to_bytes {
    ($arg:ty) => {
        impl ToBytes for $arg {
            fn to_bytes(&self) -> Vec<u8> {
                let buf = self.to_ne_bytes();
                Vec::from(buf)
            }
        }
    };
    ($arg:ty, $($args:ty),+) => {
        impl_to_bytes!($arg);
        impl_to_bytes!($($args),+);
    };
}

impl_from_bytes!(u8, i8, u16, i16, u32, i32, u64, i64);
impl_to_bytes!(u8, i8, u16, i16, u32, i32, u64, i64);


// int128 features: sense literally no one writes/reads a int128, make it a feature for the insane.

#[cfg(feature = "i128")]
impl_from_bytes!(i128);
#[cfg(feature = "i128")]
impl_to_bytes!(i128);
#[cfg(feature = "u128")]
impl_from_bytes!(u128);
#[cfg(feature = "u128")]
impl_to_bytes!(u128);

// intsize features: sense isize and usize align to either i32 or i64, make it a feature.

#[cfg(feature = "isize")]
impl_from_bytes!(isize);
#[cfg(feature = "isize")]
impl_to_bytes!(isize);
#[cfg(feature = "usize")]
impl_from_bytes!(usize);
#[cfg(feature = "usize")]
impl_to_bytes!(usize);