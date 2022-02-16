use std::io::*;
use std::mem::*;
use std::*;

use crate::*;
use byteorder::*;

pub fn systemendian() -> ByteOrder {
    return ByteOrder::new();
}

pub struct EndianReader<'a, R> where R: Read + Seek {
    pub reader: &'a mut R,
    pub order: &'a ByteOrder,
    pub encoder: &'a dyn Encoding,
}

impl<'a, R> EndianReader<'a, R> where R : Read + Seek {
    pub fn new(stream: &'a mut R, endian: &'a ByteOrder, encoding: &'a dyn Encoding) -> Self {
        return Self {
            reader: stream,
            order: endian,
            encoder: encoding
        };
    }
    fn readnumeric<N : Copy>(&mut self) -> N {
        let size = size_of::<N>();
        let mut vec = self.readbytes(size);
        if *self.order != systemendian() {
            vec = vec.into_iter().rev().collect();
        }
        let ptr = vec.as_ptr();
        let res = ptr as *const N;
        unsafe {
            return *res;
        }
    }
    pub fn readbytes<S: Into<usize>>(&mut self, len: S) -> Vec<u8> {
        let mut vec: Vec<u8> = vec![0; len.into()];
        self.reader.read(&mut vec).unwrap();
        return vec;
    }
    pub fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        return self.reader.seek(pos);
    }
    pub fn readint8(&mut self) -> i8 {
        return self.readbytes(1 as u8)[0] as i8;
    }
    pub fn readuint8(&mut self) -> u8 {
        return self.readbytes(1 as u8)[0];
    }
    pub fn readint16(&mut self) -> i16 {
        return self.readnumeric::<i16>();
    }
    pub fn readuint16(&mut self) -> u16 {
        return self.readnumeric::<u16>();
    }
    pub fn readint32(&mut self) -> i32 {
        return self.readnumeric::<i32>();
    }
    pub fn readuint32(&mut self) -> u32 {
        return self.readnumeric::<u32>();
    }
    pub fn readint64(&mut self) -> i64 {
        return self.readnumeric::<i64>();
    }
    pub fn readuint64(&mut self) -> u64 {
        return self.readnumeric::<u64>();
    }
    pub fn readstring<S : Into<usize>>(&mut self, len: S) -> String {
        let buf = self.readbytes(len);
        return self.readstringfromvec(&buf);
    }
    fn readstringfromvec(&self, buf: &[u8]) -> String {
        return self.encoder.decode(buf, DecoderTrap::Ignore).unwrap();
    }
    pub fn readzeroteminatedstring(&mut self) -> String {
        let mut vec: Vec<u8> = Vec::new();
        let mut b = self.readuint8();
        while b != 0 {
            vec.push(b);
            b = self.readuint8();
        }
        return self.readstringfromvec(&vec);
    }
    pub fn position(&mut self) -> u64 {
        return self.seek(SeekFrom::Current(0)).unwrap();
    }
}