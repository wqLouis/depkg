use std::io::{BufReader, Cursor, Read, Seek};

struct Signature {
    // https://en.wikipedia.org/wiki/List_of_file_signatures
    table: Vec<(Vec<u8>, &'static str)>, // (pattern, extension)
}

impl Signature {
    pub fn new() -> Signature {
        let mut sig = Signature { table: Vec::new() };
        sig.table.push((vec![0xff, 0xd8, 0xff], "jpg"));
        sig.table
            .push((vec![0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a], "png"));
        sig.table.push((vec![0x42, 0x4d], "bmp"));

        sig
    }

    pub fn match_extension(&self, bytes: &[u8]) -> &str {
        for (match_bytes, extension) in &self.table {
            let mut valid = true;
            for (byte, match_byte) in bytes.iter().zip(match_bytes) {
                if byte != match_byte {
                    valid = false;
                    break;
                }
            }
            if valid {
                return extension;
            }
        }
        &"tex" // no match
    }
}

pub fn parse(bytes: &Vec<u8>) -> (Vec<u8>, String) {
    const MAGIC: usize = 8;
    const SEP: i64 = 1;
    const TEX_SIZE: usize = 4;

    let mut buf = BufReader::new(Cursor::new(bytes));

    let mut texv = [0u8; MAGIC]; // I have no idea what all this magic variables are
    let mut texi = [0u8; MAGIC];
    let mut texb = [0u8; MAGIC];
    let mut size = [0u8; TEX_SIZE];
    let mut dimension: [[u8; TEX_SIZE]; 2] = [[0u8; TEX_SIZE]; 2]; // w h
    let mut payload: Vec<u8>;

    buf.read_exact(&mut texv).unwrap();
    buf.seek_relative(SEP).unwrap();
    buf.read_exact(&mut texi).unwrap();
    buf.seek_relative(SEP + MAGIC as i64).unwrap();
    buf.read_exact(&mut dimension[0]).unwrap();
    buf.read_exact(&mut dimension[1]).unwrap();
    buf.seek_relative(SEP + (TEX_SIZE * 3) as i64).unwrap();
    buf.read_exact(&mut texb).unwrap();
    buf.seek_relative((MAGIC * 4) as i64 + SEP).unwrap();
    buf.read_exact(&mut size).unwrap();

    payload = vec![0u8; u32::from_le_bytes(size) as usize];

    buf.read_exact(&mut payload).unwrap();

    let sig = Signature::new();

    if payload.len() < 8 {
        panic!("Broken tex file with too small payload");
    }
    if String::from_utf8_lossy(&texb) == String::from("TEXB0003") {}

    let extension = sig.match_extension(&payload[0..8]).to_owned();

    if extension == "tex" {
        // if no match logics
        return (bytes.to_vec(), extension);
    }

    (payload, extension)
}
