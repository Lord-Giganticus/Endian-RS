use crate::{endian::*, bytes::*, encoding::{Encoding, EncoderTrap::Ignore}};

use std::io::{Write, Seek};

pub trait SeekWrite: Write + Seek {}

impl<T : Write + Seek> SeekWrite for T {}

pub struct Writer<W: SeekWrite> {
    pub stream: W,
    pub endian: Endian
}

impl<W: SeekWrite> Write for Writer<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.stream.write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.stream.flush()
    }
}

impl<W: SeekWrite> Seek for Writer<W> {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        self.stream.seek(pos)
    }
}

impl<W: SeekWrite + Clone> Clone for Writer<W> {
    fn clone(&self) -> Self {
        Self::new(self.stream.clone(), self.endian.clone())
    }
}

impl<W: SeekWrite> Writer<W> {
    pub fn new(stream: W, endian: Endian) -> Self {
        Self { stream, endian }
    }
    pub fn write_numeric<N: Numeric>(&mut self, num: N) -> std::io::Result<usize> {
        let mut buf = num.to_bytes();
        if self.endian != NATIVE {
            buf.reverse();
        }
        self.write(&buf)
    }
    pub fn write_string<E: Encoding>(&mut self, msg: &str, enc: &E) -> std::io::Result<usize> {
        let buf = enc.encode(msg.into(), Ignore).unwrap_or_default();
        self.write(&buf)
    }
    pub fn write_zero_terminated_string<E: Encoding>(&mut self, msg: &str, enc: &E) -> std::io::Result<usize> {
        let mut zts = String::from(msg);
        zts += "\0";
        self.write_string(&zts, enc)
    }
    
}