/// Public file for macro defs

macro_rules! to_bytes {
    ($a:expr,$b:ty)=>{
        {
            let pointer: *const $b = &$a;
            let raw = pointer as *const u8;
            unsafe {
                std::slice::from_raw_parts(raw, std::mem::size_of::<$b>()).to_vec()
            }
        }
    }
}

macro_rules! to_type {
    ($a:expr,$b:ty)=>{
        {
            let pointer = $a.as_ptr();
            let res = pointer as *const $b;
            unsafe {
                *res
            }
        }
    }
}


#[macro_export]
macro_rules! reverse {
    ($buf:expr) => {
        {
            $buf = $buf.into_iter().rev().collect();
        }
    }
}

pub trait Data {
    fn as_bytes(self) -> Vec<u8>;
    fn from_bytes(buf: Vec<u8>) -> Self;
}

macro_rules! impl_bytes {
    ($arg:ty) => {
        impl Data for $arg {
            fn as_bytes(self) -> Vec<u8> {
                to_bytes!(self, $arg)
            }
            fn from_bytes(buf: Vec<u8>) -> Self {
                to_type!(buf, $arg)
            }
        }
    };
}

impl_bytes!(i8);
impl_bytes!(u8);
impl_bytes!(i16);
impl_bytes!(u16);
impl_bytes!(i32);
impl_bytes!(u32);
impl_bytes!(i64);
impl_bytes!(u64);
impl_bytes!(f32);
impl_bytes!(f64);