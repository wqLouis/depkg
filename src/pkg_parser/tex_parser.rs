use std::io::{BufReader, Cursor, Read};

pub fn parse(bytes: &Vec<u8>) -> Vec<u8> {
    const MAGIC: usize = 8;
    const SEP: i64 = 1;
    const TEX_SIZE: usize = 4;

    let mut buf = BufReader::new(Cursor::new(bytes));

    let mut texv = [0u8; MAGIC]; // I have no idea what all this magic variables are
    let mut texi = [0u8; MAGIC];
    let mut texb = [0u8; MAGIC];
    let mut size = [0u8; TEX_SIZE];
    let mut payload: Vec<u8>;

    buf.read_exact(&mut texv).unwrap();
    buf.seek_relative(SEP).unwrap();
    buf.read_exact(&mut texi).unwrap();
    buf.seek_relative(SEP + (TEX_SIZE * 7) as i64).unwrap();
    buf.read_exact(&mut texb).unwrap();
    buf.seek_relative((MAGIC * 4) as i64 + SEP).unwrap();
    buf.read_exact(&mut size).unwrap();

    payload = vec![0u8; u32::from_le_bytes(size) as usize];

    buf.read_exact(&mut payload).unwrap();

    payload
}
