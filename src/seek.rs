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

pub trait TempSeek: Seek {
    fn temp_seek<F: FnOnce(&mut Self) -> ()>(&mut self, pos: SeekFrom, func: F) {
        let p = self.seek(SeekFrom::Current(0)).unwrap_or_default();
        self.seek(pos).unwrap();
        func(self);
        self.seek(SeekFrom::Start(p)).unwrap();
    }
}

impl<S: Seek> TempSeek for S {}