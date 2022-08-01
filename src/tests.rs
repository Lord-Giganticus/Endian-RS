#[cfg(test)]
mod tests {
    use crate::*;
    use std::io::Cursor;
    use std::io::SeekFrom;
    #[test]
    fn ztstest() {
        let vec = vec![b'Y', b'a', b'z', b'0'];
        let stream = Cursor::new(vec);
        let mut reader = reader::Reader::new(stream, endian::NATIVE);
        let mut num = 0i32;
        seek::temp_seek(&mut reader, SeekFrom::Start(0), &mut |s| {
            num = s.read_numeric().unwrap_or_default();
        });
        println!("{}", num);
    }
}
