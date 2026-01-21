use std::io::{Cursor, Read, Seek, SeekFrom};

pub fn parse(bytes: &[u8]) -> (&str, Vec<u8>) {
    let mut cursor = Cursor::new(bytes);
}
