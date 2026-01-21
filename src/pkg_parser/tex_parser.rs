use std::io::{BufReader, Cursor, Read, Seek};

struct Signature {
    // https://en.wikipedia.org/wiki/List_of_file_signatures
    table: Vec<(Vec<u32>, &'static str)>, // (pattern, extension)
}

impl Signature {
    pub fn new() -> Signature {
        let mut sig = Signature { table: Vec::new() };
        sig.table.push((vec![0xff, 0xd8, 0xff], "jpg"));
        sig.table
            .push((vec![0x89, 0x48, 0x44, 0x47, 0x0d, 0x0a, 0x1a, 0x0a], "png"));

        sig
    }

    pub fn match_extension(bytes: &Vec<u8>) {}

    fn vec_u8_to_u32(bytes: &[u8]) -> Vec<u32> {
        bytes
            .chunks_exact(4)
            .map(|chunk| u32::from_le_bytes(chunk.try_into().unwrap()))
            .collect()
    }
}

pub fn parse(bytes: &Vec<u8>, parse_mipmap: bool) -> Vec<Vec<u8>> {
    const MAGIC: usize = 8;
    const SEP: i64 = 1;
    const TEX_SIZE: usize = 4;

    let mut buf = BufReader::new(Cursor::new(bytes));

    let mut texv = [0u8; MAGIC]; // I have no idea what all this magic variables are
    let mut texi = [0u8; MAGIC];
    let mut texb = [0u8; MAGIC];
    let mut size = [0u8; TEX_SIZE];
    let mut payload: Vec<u8>;
    let mut payloads: Vec<Vec<u8>> = Vec::new();

    buf.read_exact(&mut texv).unwrap();
    buf.seek_relative(SEP).unwrap();
    buf.read_exact(&mut texi).unwrap();
    buf.seek_relative(SEP + (TEX_SIZE * 7) as i64).unwrap();
    buf.read_exact(&mut texb).unwrap();
    buf.seek_relative((MAGIC * 4) as i64 + SEP).unwrap();
    buf.read_exact(&mut size).unwrap();

    payload = vec![0u8; u32::from_le_bytes(size) as usize];

    buf.read_exact(&mut payload).unwrap();

    payloads.push(payload.clone());

    if !parse_mipmap {
        return payloads;
    }

    let bytes_len = bytes.len();

    // read mip map
    loop {
        let pos = buf.stream_position().unwrap();
        if bytes_len == pos as usize {
            break;
        }
        buf.seek_relative((MAGIC * 2) as i64).unwrap();
        buf.read_exact(&mut size).unwrap();
        let size = u32::from_le_bytes(size);
        if size == 0 {
            break;
        }
        payload = vec![0u8; size as usize];
        buf.read_exact(&mut payload).unwrap();
        payloads.push(payload.clone());
    }

    payloads
}
