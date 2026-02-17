use bitflags::bitflags;

bitflags! {
    struct VTFFlags: u32 {
        // TODO
    }
}

// TODO: needs data field
#[derive(Debug, Eq, PartialEq)]
struct ResourceEntry {
    pub tag: String,
    pub flags: u8,
    pub offset: u32,
}

// TODO: needs vtf data field
#[derive(Debug, PartialEq)]
struct VTF {
    pub version: f32,
    pub width: u16,
    pub height: u16,
    /// See VTFFlags
    pub flags: u32,
    pub frames: u16,
    pub first_frame: u16,
    pub reflectivity: f32,
    pub bumpmap_scale: f32,
    pub high_res_image_format: u32,
    pub mipmapcount: u8,
    pub low_res_image_format: u32,
    pub low_res_image_width: u32,
    pub low_res_image_height: u32,
    /// Always none when version < 7.2
    pub depth: Option<u16>,
    /// Always none when version < 7.3
    pub num_resources: u32,

    pub resource_entries: Vec<ResourceEntry>,
}
