use std::io::{BufReader, Cursor, Read};

fn match_sig(bytes: [u8; 8]) -> String {
    const PNG_SIG: ([u8; 8], &str) = ([0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a], "png");
    const JPG_SIG: ([u8; 3], &str) = ([0xff, 0xd8, 0xff], "jpg");

    if bytes == PNG_SIG.0 {
        return PNG_SIG.1.to_owned();
    }
    if bytes[..3] == JPG_SIG.0 {
        return JPG_SIG.1.to_owned();
    }

    return "tex".to_owned();
}

fn match_format(bytes: [u8; 4]) -> String {
    let bytes = u32::from_le_bytes(bytes);
    match bytes {
        0 => return "raw".to_owned(),
        4 | 7 => return "dxt1".to_owned(),
        6 => return "dxt5".to_owned(),
        8 => return "rg88".to_owned(),
        9 => return "r8".to_owned(),
        _ => return "unknown".to_owned(),
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
    let mut dimension = [[0u8; TEX_SIZE]; 2]; // w h
    let mut format = [0u8; TEX_SIZE];
    let mut payload: Vec<u8>;

    buf.read_exact(&mut texv).unwrap();
    buf.seek_relative(SEP).unwrap();
    buf.read_exact(&mut texi).unwrap();
    buf.seek_relative(SEP).unwrap();
    buf.read_exact(&mut format).unwrap();
    buf.seek_relative(TEX_SIZE as i64).unwrap();
    buf.read_exact(&mut dimension[0]).unwrap();
    buf.read_exact(&mut dimension[1]).unwrap();
    buf.seek_relative((TEX_SIZE * 3) as i64).unwrap();
    buf.read_exact(&mut texb).unwrap();
    buf.seek_relative((MAGIC * 4) as i64 + SEP).unwrap();
    buf.read_exact(&mut size).unwrap();

    payload = vec![0u8; u32::from_le_bytes(size) as usize];

    buf.read_exact(&mut payload).unwrap();

    let mut padded_arr = [0u8; 8];
    let payload_len = std::cmp::min(8, payload.len());
    padded_arr[..payload_len].copy_from_slice(&payload[..payload_len]);
    let extension = match_sig(padded_arr);

    if String::from_utf8_lossy(&texb) == String::from("TEXB0003") {}

    if extension == "tex" {
        // if no match logics

        println!("{}", match_format(format));

        return (bytes.to_vec(), extension);
    }

    (payload, extension)
}
