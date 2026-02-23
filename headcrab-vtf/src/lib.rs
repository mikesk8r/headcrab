use std::num;

use bitflags::bitflags;
use scroll::{Pread, ctx};

bitflags! {
    struct VTFFlags: u32 {
        const VTF_POINTSAMPLE = 0x00000001;
        const VTF_TRILINEAR = 0x00000002;
        const VTF_CLAMPS = 0x00000004;
        const VTF_CLAMPT = 0x00000008;
        // todo...
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ImageDataFormat {
    #[default]
    Unknown,
    ARGB8888,
    DXT1,
}

pub fn get_format_from_id(format: i32) -> ImageDataFormat {
    use ImageDataFormat::*;

    return match format {
        -1 => Unknown,
        11 => ARGB8888,
        13 => DXT1,
        _ => Unknown,
    };
}

fn get_color(format: &ImageDataFormat, bytes: &[u8]) -> (f32, f32, f32, f32) {
    use ImageDataFormat::*;

    return match format {
        Unknown => (0.0, 0.0, 0.0, 0.0),
        ARGB8888 => {
            let alpha: u8 = bytes.pread(0).unwrap();
            let red: u8 = bytes.pread(0).unwrap();
            let green: u8 = bytes.pread(0).unwrap();
            let blue: u8 = bytes.pread(0).unwrap();

            (alpha.into(), red.into(), green.into(), blue.into())
        }
        DX1 => {
            // todo
            (0.0, 0.0, 0.0, 0.0)
        }
    };
}

#[derive(Debug, PartialEq)]
struct ImageData {
    pub is_hi_res: bool,
    /// The image's data in pixels.
    /// It is stored as a list of rows of (red, green, blue, alpha)
    pub data: Vec<Vec<(f32, f32, f32, f32)>>,
}

#[derive(Debug, Eq, PartialEq)]
struct ResourceEntry {
    pub tag: (u8, u8, u8),
    pub flags: u8,
    pub offset: u32,
    // TODO: maybe add field for data? not really sure
}

#[derive(Debug, Default, PartialEq)]
struct VTF {
    pub version: f32,
    pub width: u16,
    pub height: u16,
    /// See VTFFlags
    pub flags: u32,
    pub frames: u16,
    pub first_frame: u16,
    pub reflectivity: (f32, f32, f32),
    pub bumpmap_scale: f32,
    pub high_res_image_format: ImageDataFormat,
    pub mipmapcount: u8,
    pub low_res_image_format: ImageDataFormat,
    pub low_res_image_width: u8,
    pub low_res_image_height: u8,
    /// Always none when version < 7.2
    pub depth: Option<u16>,

    pub resource_entries: Vec<ResourceEntry>,
    pub image_data: Vec<ImageData>,
}

impl VTF {
    fn from_bytes(bytes: &[u8]) -> VTF {
        let mut vtf = VTF::default();

        let ver_major: u32 = bytes.pread(4).unwrap();
        let ver_minor: u32 = bytes.pread(8).unwrap();
        vtf.version = format!("{}.{}", ver_major, ver_minor)
            .parse::<f32>()
            .unwrap();
        let header_size: u32 = bytes.pread(12).unwrap();
        vtf.width = bytes.pread(16).unwrap();
        vtf.height = bytes.pread(18).unwrap();
        vtf.flags = bytes.pread(20).unwrap();
        vtf.frames = bytes.pread(24).unwrap();
        vtf.first_frame = bytes.pread(26).unwrap();
        let refl_1: f32 = bytes.pread(32).unwrap();
        let refl_2: f32 = bytes.pread(36).unwrap();
        let refl_3: f32 = bytes.pread(40).unwrap();
        vtf.reflectivity = (refl_1, refl_2, refl_3);
        vtf.bumpmap_scale = bytes.pread(48).unwrap();
        vtf.high_res_image_format = get_format_from_id(bytes.pread(52).unwrap());
        vtf.mipmapcount = bytes.pread(56).unwrap();
        vtf.low_res_image_format = get_format_from_id(bytes.pread(57).unwrap());
        vtf.low_res_image_width = bytes.pread(61).unwrap();
        vtf.low_res_image_height = bytes.pread(62).unwrap();
        if vtf.version >= 7.2 {
            vtf.depth = Some(bytes.pread(63).unwrap());
        }
        if vtf.version >= 7.3 {
            let num_resources = bytes.pread::<u32>(68).unwrap();
            let mut i = 0;
            while num_resources > 0 && i < num_resources {
                let pos: usize = (80 + i * 8).try_into().unwrap();
                let tag = (
                    bytes.pread::<u8>(pos).unwrap(),
                    bytes.pread::<u8>(pos + 1).unwrap(),
                    bytes.pread::<u8>(pos + 2).unwrap(),
                );
                let flags = bytes.pread::<u8>(pos + 3).unwrap();
                let offset = bytes.pread::<u32>(pos + 4).unwrap();

                match tag {
                    (0x01, 0x00, 0x00) | (0x30, 0x00, 0x00) => {
                        use ImageDataFormat::*;

                        let hi_res = match tag.0 {
                            0x30 => true,
                            _ => false,
                        };
                        let format = if hi_res == true {
                            vtf.high_res_image_format
                        } else {
                            vtf.low_res_image_format
                        };
                        let height = if hi_res == true {
                            vtf.height
                        } else {
                            vtf.low_res_image_height.into()
                        };
                        let width = if hi_res == true {
                            vtf.width
                        } else {
                            vtf.low_res_image_width.into()
                        };
                        let read_length = match format {
                            ARGB8888 => 4,
                            _ => 0,
                        };
                        let mut buffer: Vec<Vec<(f32, f32, f32, f32)>> = vec![];
                        let mut j = 0;
                        let mut k = 0;
                        let mut l = 0;
                        // dear god this is awful :sob:
                        while j < height {
                            let mut row: Vec<(f32, f32, f32, f32)> = vec![];
                            while k < width {
                                let mut color: Vec<u8> = vec![];
                                while l < read_length {
                                    let pos: usize = (((j * vtf.width * read_length)
                                        + k * read_length)
                                        + l
                                        + offset as u16)
                                        .into();
                                    color.push(bytes.pread(pos).unwrap());
                                    l += 1;
                                }
                                row.push(get_color(&format, color.as_slice()));
                                l = 0;
                                k += 1;
                            }
                            buffer.push(row);
                            k = 0;
                            j += 1;
                        }

                        vtf.image_data.push(ImageData {
                            is_hi_res: hi_res,
                            data: buffer,
                        })
                    }
                    _ => vtf.resource_entries.push(ResourceEntry {
                        tag: tag,
                        flags: flags,
                        offset: offset,
                    }),
                }

                i += 1;
            }
        }

        vtf
    }
}
