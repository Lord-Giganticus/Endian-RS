use std::io::{Seek, SeekFrom};

pub trait SeekBegin : Seek {
    fn seekbegin(&mut self) -> std::io::Result<u64> {
        self.seek(SeekFrom::Start(0))
    }
}

pub trait SeekEnd: Seek {
    fn seekend(&mut self) -> std::io::Result<u64> {
        self.seek(SeekFrom::End(0))
    } 
}

impl<S: Seek> SeekBegin for S {}
impl<S: Seek> SeekEnd for S {}

pub fn temp_seek<S: Seek, F: FnMut(&mut S) -> ()>(stream: &mut S, pos: SeekFrom, func: &mut F) {
    let p = stream.seek(SeekFrom::Current(0)).unwrap_or_default();
    stream.seek(pos).unwrap();
    (func)(stream);
    stream.seek(SeekFrom::Start(p)).unwrap();
}