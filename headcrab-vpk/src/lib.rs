use scroll::Pread;
use std::collections::HashMap;

mod error;

pub use error::*;

#[derive(Debug, Default)]
pub struct VPK {
    pub version: u8,
    // the string is the entry's path in the VPK
    pub entries: HashMap<String, VPKEntry>,
}

impl VPK {
    /// Try to read an entry from the VPK.
    pub fn get<T>(&self, key: T) -> Option<&VPKEntry>
    where
        T: Sized + Eq + std::hash::Hash,
        String: std::borrow::Borrow<T>,
    {
        self.entries.get(&key)
    }

    pub fn get_mut<T>(&mut self, key: T) -> Option<&mut VPKEntry>
    where
        T: Sized + Eq + std::hash::Hash,
        String: std::borrow::Borrow<T>,
    {
        self.entries.get_mut(&key)
    }

    /// Read a VPK from its `_dir` file.
    pub fn from_path(path: String) -> Result<VPK, Error> {
        let file = std::fs::read(&path);
        let split = path.split('_').collect::<Vec<&str>>();

        if let Ok(dir) = file {
            let bytes = dir.as_slice();

            if bytes.pread::<u32>(0).unwrap() != 0x55aa1234 {
                return Err(Error::InvalidHeader);
            }

            let mut vpk = VPK {
                version: bytes.pread::<u32>(4).unwrap() as u8,
                entries: std::collections::HashMap::new(),
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

                let file = format!(
                    "{}_{}{}.vpk",
                    split[0],
                    "0".repeat(3 - archive_index.checked_ilog10().unwrap_or(0) as usize),
                    archive_index
                );

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
                    on_disk: true,
                    contents: None,
                    file: file,
                    offset: entry_offset,
                    len: entry_length,
                };

                vpk.entries.insert(entry.path.clone(), entry);
            }

            Ok(vpk)
        } else {
            return Err(Error::CannotFindFile);
        }
    }

    pub fn write(&self, buffer: &[u8]) -> Result<(), Error> {
        todo!()
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
    pub on_disk: bool,
    pub contents: Option<Vec<u8>>,
    file: String,
    offset: u32,
    len: u32,
}

impl VPKEntry {
    pub fn new(entry_type: EntryType, path: String, data: Vec<u8>) -> Self {
        VPKEntry {
            entry_type,
            path,
            on_disk: false,
            contents: Some(data),
            file: "".to_string(),
            offset: 0,
            len: 0,
        }
    }

    /// Reads the VPK entry from disk if it's not already loaded.
    pub fn read(&mut self) -> Result<(), Error> {
        let file = std::fs::read(&self.file);

        if self.contents.is_some() {
            return Ok(());
        }

        if let Ok(bytes) = file {
            let data: Vec<u8> =
                bytes[self.offset as usize..self.offset as usize + self.len as usize].into();

            self.contents = Some(data);

            Ok(())
        } else {
            return Err(Error::CannotFindFile);
        }
    }

    /// Uncaches the contents of the entry.
    pub fn uncache(&mut self) {
        self.contents = None;
    }
}
