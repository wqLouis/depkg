use std::{fs::File, io::Read, path::Path};

struct Header {
    version: String,
    file_count: u32,
}

pub struct Pkg {
    file: File,
    pub header: Header,
}

impl Pkg {
    pub fn new(pkg_path: &Path) -> Result<Pkg> {
        let mut file = File::open(pkg_path)?;
        let header_meta = Self::read_header(&mut file);
        let header = Header {
            version: header_meta.0,
            file_count: header_meta.1,
        };

        Ok(Pkg { file, header })
    }

    fn read_header(file: &mut File) -> (String, u32) {
        // Read the header of the pkg

        const HEADER_LEN: usize = 4;
        const FILE_COUNT_LEN: usize = 4;

        fn header_len(file: &mut File) -> u32 {
            let mut header_len = [0u8; HEADER_LEN];
            file.read_exact(&mut header_len).unwrap();
            u32::from_le_bytes(header_len)
        }

        fn header_version(file: &mut File, len: usize) -> String {
            let mut header_v = vec![0u8; len];
            file.read_exact(&mut header_v).unwrap();
            String::from_utf8(header_v).unwrap()
        }

        fn file_count(file: &mut File) -> u32 {
            let mut file_count = [0u8; FILE_COUNT_LEN];
            file.read_exact(&mut file_count).unwrap();
            u32::from_le_bytes(file_count)
        }

        let len = header_len(file);
        let version = header_version(file, len as usize);
        let file_count = file_count(file);

        (version, file_count)
    }
}
