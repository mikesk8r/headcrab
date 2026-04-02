use scroll::Pread;

mod error;

pub use error::*;

#[derive(Debug, Default)]
pub struct VPK {
    pub version: u8,
    pub entries: Vec<VPKEntry>,
}

impl VPK {
    /// Read a VPK from its `_dir` file.
    pub fn from_vpk(path: String) -> Result<VPK, Error> {
        let file = std::fs::read(&path);
        let split = path.split('_').collect::<Vec<&str>>();

        if let Ok(dir) = file {
            let bytes = dir.as_slice();

            if bytes.pread::<u32>(0).unwrap() != 0x55aa1234 {
                return Err(Error::InvalidHeader);
            }

            let mut vpk = VPK {
                version: bytes.pread::<u32>(4).unwrap() as u8,
                entries: vec![],
            };

            let mut pos = if vpk.version == 1 { 12 } else { 28 };
            let mut last_extension: Option<&str> = None;
            let mut last_path: Option<&str> = None;
            while pos != bytes.len() - 1 {
                let (extension, new_pos) = if let Some(ext) = last_extension {
                    (ext, pos)
                } else {
                    read_string(bytes, pos)
                };
                let (path, new_pos) = if let Some(path) = last_path {
                    (path, new_pos)
                } else {
                    read_string(bytes, new_pos)
                };
                let (filename, new_pos) = read_string(bytes, new_pos);
                pos = new_pos;

                // let crc: u32 = bytes.pread(pos).unwrap();
                // TODO
                // let preload_bytes: u16 = bytes.pread(pos + 4).unwrap();
                let archive_index: u16 = bytes.pread(pos + 6).unwrap();
                let entry_offset: u32 = bytes.pread(pos + 8).unwrap();
                let entry_length: u32 = bytes.pread(pos + 12).unwrap();

                if bytes.pread::<u32>(pos + 16).unwrap() == 65535 {
                    pos += 20;
                    last_extension = None;
                    last_path = None;
                } else {
                    pos += 18;
                    last_extension = Some(extension);
                    last_path = Some(path);
                }

                let file = if archive_index < 10 {
                    format!("{}_00{}.vpk", split[0], archive_index)
                } else if archive_index < 100 {
                    format!("{}_0{}.vpk", split[0], archive_index)
                } else {
                    format!("{}_{}.vpk", split[0], archive_index)
                };

                let entry = VPKEntry {
                    entry_type: match extension {
                        "vtf" => EntryType::VTF,
                        "kv3" => EntryType::KeyValues3,
                        _ => EntryType::Other,
                    },
                    path: if path == " " {
                        format!("{}.{}", filename, extension)
                    } else {
                        format!("{}/{}.{}", path, filename, extension)
                    },
                    file: file,
                    offset: entry_offset,
                    len: entry_length,
                };

                vpk.entries.push(entry);
            }

            Ok(vpk)
        } else {
            return Err(Error::CannotFindFile);
        }
    }
}

fn read_string(bytes: &[u8], pos: usize) -> (&str, usize) {
    let mut end_pos = pos;
    while bytes[end_pos] != 0x00 {
        end_pos += 1;
    }

    (str::from_utf8(&bytes[pos..end_pos]).unwrap(), end_pos + 1)
}

#[derive(Clone, Debug)]
pub enum EntryType {
    Other,
    VTF,
    // add mdl types
    KeyValues1,
    KeyValues2,
    KeyValues3,
}

#[derive(Clone, Debug)]
pub struct VPKEntry {
    pub entry_type: EntryType,
    pub path: String,
    file: String,
    offset: u32,
    len: u32,
}

impl VPKEntry {
    pub fn read(&self) -> Result<Vec<u8>, Error> {
        let file = std::fs::read(&self.path);

        if let Ok(bytes) = file {
            return Ok(
                bytes[self.offset as usize..self.offset as usize + self.len as usize].into(),
            );
        } else {
            return Err(Error::CannotFindFile);
        }
    }
}
