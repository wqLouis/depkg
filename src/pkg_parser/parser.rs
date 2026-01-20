use std::{fs::File, io::Read, path::Path};

pub struct Pkg {
    file: File,
    pub header: Header,
    pub entries: Vec<Entry>,
}

struct Header {
    version: String,
    file_count: u32,
}

struct Entry {
    path: String,
    offset: u32,
    size: u32,
}

impl Pkg {
    pub fn new(pkg_path: &Path) -> Pkg {
        let mut file = File::open(pkg_path).unwrap();
        let header_meta = Self::read_header(&mut file);
        let header = Header {
            version: header_meta.0,
            file_count: header_meta.1,
        };
        let entries = Vec::<Entry>::new();

        Pkg {
            file,
            header,
            entries,
        }
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
        let count = file_count(file);

        (version, count)
    }

    fn read_entry(&mut self, entry_count: u64) {
        // Read the file entry of pkg

        const PATH_LEN: usize = 4;
        const DATA_OFFSET: usize = 4;
        const DATA_SIZE: usize = 4;

        let mut path_len = [0u8; PATH_LEN];
        let mut data_offset = Vec::<u32>::new();
        let mut data_size = Vec::<u32>::new();
        let mut path = Vec::<String>::new();

        let mut file = &self.file;

        for _ in 0..entry_count {
            file.read_exact(&mut path_len).unwrap();

            let mut path_buffer = vec![0u8; u32::from_le_bytes(path_len) as usize];
            let mut data_offset_buffer = [0u8; DATA_OFFSET];
            let mut data_size_buffer = [0u8; DATA_SIZE];

            file.read_exact(&mut path_buffer).unwrap();
            file.read_exact(&mut data_offset_buffer).unwrap();
            file.read_exact(&mut data_size_buffer).unwrap();

            path.push(String::from_utf8(path_buffer).unwrap());
            data_offset.push(u32::from_le_bytes(data_offset_buffer));
            data_size.push(u32::from_le_bytes(data_size_buffer));
        }
    }
}
