use std::io::*;
use std::*;

use crate::*;
use byteorder::*;
use endianreader::*;

pub struct EndianWriter<'a, W> where W : Write + Seek {
    pub writer: &'a mut W,
    pub order: &'a ByteOrder,
    pub encoder: &'a dyn Encoding,
}

impl<'a, W> EndianWriter<'a, W> where W : Write + Seek {
    pub fn new(stream: &'a mut W, endian: &'a ByteOrder, encoding: &'a dyn Encoding) -> Self{
        return Self {
            writer: stream,
            order: endian,
            encoder: encoding
        }
    }
    fn writenumeric<N : Copy + macros::Data>(&mut self, num: N) -> usize {
        let mut buf = num.as_bytes();
        if self.reverse() {
            reverse!(buf);
        }
        return self.writebytes(&buf);
    }
    pub fn writebytes(&mut self, data: &[u8]) -> usize {
        return self.writer.write(data).unwrap();
    }
    pub fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        return self.writer.seek(pos);
    }
    pub fn writeint8(&mut self, num: i8) -> usize {
        return self.writenumeric(num);
    }
    pub fn writeuint8(&mut self, num: u8) -> usize {
        return self.writenumeric(num);
    }
    pub fn writeint16(&mut self, num: i16) -> usize {
        return self.writenumeric(num);
    }
    pub fn writeuint16(&mut self, num: u16) -> usize {
        return self.writenumeric(num);
    }
    pub fn writeint32(&mut self, num: i32) -> usize {
        return self.writenumeric(num);
    }
    pub fn writeuint32(&mut self, num: u32) -> usize {
        return self.writenumeric(num);
    }
    pub fn writeint64(&mut self, num: i64) -> usize {
        return self.writenumeric(num);
    }
    pub fn writeuint64(&mut self, num: u64) -> usize {
        return self.writenumeric(num);
    }
    pub fn writesingle(&mut self, num: f32) -> usize {
        return self.writenumeric(num);
    }
    pub fn writedouble(&mut self, num: f64) -> usize {
        return self.writenumeric(num);
    }
    pub fn writestring(&mut self, data: String) -> usize {
        let buf = self.encoder.encode(&data, EncoderTrap::Ignore).unwrap();
        return self.writebytes(&buf);
    }
    pub fn changeorder(&mut self, order: &'a ByteOrder) {
        self.order = order;
    }
    pub fn reverse(&self) -> bool {
        *self.order == systemendian()
    }
}