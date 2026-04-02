#[derive(Debug, Default)]
pub struct VPK {
    pub version: u8,
    pub entries: Vec<VPKEntry>,
}

#[derive(Clone, Debug)]
pub enum EntryType {
    VTF,
    // add mdl types
    KeyValues1,
    KeyValues2,
    KeyValues3,
}

#[derive(Clone, Debug)]
pub struct VPKEntry {
    pub entry_type: EntryType,
    file: String,
    offset: u32,
}

impl VPKEntry {
    pub fn read() /* -> Vec<u8> */
    {
        // TODO
    }
}
