use crate::{endian::*, bytes::*, encoding::{Encoding, DecoderTrap::Ignore}};

use std::io::{Read, Seek};

pub trait SeekRead : Read + Seek {}

impl<T: Read + Seek> SeekRead for T {}

pub struct Reader<R: SeekRead> {
    pub stream: R,
    pub order: Endian
}

impl<R: SeekRead> Read for Reader<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.stream.read(buf)
    }
}

impl<R: SeekRead> Seek for Reader<R> {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        self.stream.seek(pos)
    }
}

impl<R: SeekRead + Clone> Clone for Reader<R> {
    fn clone(&self) -> Self {
        Self::new(self.stream.clone(), self.order.clone())
    }
}

impl<R: SeekRead> Reader<R> {
    pub fn new(stream: R, order: Endian) -> Self {
        Self {stream, order}
    }
    pub fn read_numeric<N: Numeric>(&mut self) -> std::io::Result<N> {
        let size = std::mem::size_of::<N>();
        let mut buf: Vec<u8> = vec![0; size];
        self.read(&mut buf)?;
        let mut num = N::from_bytes(&buf);
        if self.order != NATIVE && size > 1 {
            num = num.reverse();
        }
        Ok(num)
    }
    pub fn read_string<E: Encoding>(&mut self, len: usize, enc: &E) -> std::io::Result<String>  {
        let mut vec = vec![0u8; len];
        self.read(&mut vec)?;
        Ok(enc.decode(&vec, Ignore).unwrap_or_default())
    }
    pub fn read_zero_terminated_string<E: Encoding>(&mut self, enc: &E) -> std::io::Result<String> {
        let mut vec: Vec<u8> = Vec::new();
        let mut b = self.read_numeric::<u8>()?;
        while b != 0 {
            vec.push(b);
            b = self.read_numeric::<u8>()?;
        }
        Ok(enc.decode(&vec, Ignore).unwrap_or_default())
    }
}