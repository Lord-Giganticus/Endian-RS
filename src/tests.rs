#[cfg(test)]
mod tests {
    use crate::*;
    use std::io::Cursor;
    #[test]
    fn ztstest() {
        let msg = "THP\0";
        let buf = msg.as_bytes();
        let stream = Cursor::new(Vec::from(buf));
        let mut reader = reader::Reader::new(stream, endian::NATIVE);
        let msg2 = reader.read_zero_terminated_string(encoding::all::ASCII).unwrap();
        println!("{}", &msg[..3] == msg2);
    }
}
