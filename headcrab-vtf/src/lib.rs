use std::fs::read;

use bitflags::bitflags;
use scroll::Pread;

mod dxt;
mod formats;

use formats::*;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ImageDataFormat {
    #[default]
    Unknown,
    RGBA8888,
    ABGR8888,
    RGB888,
    BGR888,
    RGB888Bluescreen,
    BGR888Bluescreen,
    ARGB8888,
    DXT1,
    DXT3,
    DXT5,
}

bitflags! {
    struct VTFFlags: u32 {
        const VTF_POINTSAMPLE = 0x00000001;
        const VTF_TRILINEAR = 0x00000002;
        const VTF_CLAMPS = 0x00000004;
        const VTF_CLAMPT = 0x00000008;
        const VTF_ANISOTROPIC = 0x00000010;
        const VTF_HINT_DXT5 = 0x00000020;
        const VTF_PWL_CORRECTED = 0x00000040;
        const VTF_NORMAL = 0x00000080;
        const VTF_NOMIP = 0x00000100;
        const VTF_NOLOD = 0x00000200;
        const VTF_ALL_MIPS = 0x00000400;
        const VTF_PROCEDURAL = 0x00000800;
        const VTF_ONEBITALPHA = 0x00001000;
        const VTF_EIGHTBITALPHA = 0x00002000;
        const VTF_ENVMAP = 0x00004000;
        const VTF_RENDERTARGET = 0x00008000;
        const VTF_DEPTHRENDERTARGET = 0x00010000;
        const VTF_NODEBUGOVERRIDE = 0x00020000;
        const VTF_SINGLECOPY = 0x00040000;
        const VTF_PRE_SRGB = 0x00080000;
        const VTF_NODEPTHBUFFER = 0x00800000;
        const VTF_CLAMPU = 0x02000000;
        const VTF_VERTEXTEXTURE = 0x04000000;
        const VTF_SSBUMP = 0x08000000;
        const VTF_BORDER = 0x20000000;
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ImageData {
    pub is_hi_res: bool,
    /// The image's data in pixels.
    /// It is stored as a list of [red, green, blue, alpha)]
    pub data: Vec<(u8, u8, u8, u8)>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourceEntry {
    pub tag: (u8, u8, u8),
    pub flags: u8,
    pub offset: u32,
    // TODO: maybe add field for data? not really sure
}

#[derive(Debug, Default, PartialEq)]
pub struct VTF {
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
    pub fn from_bytes(bytes: &[u8]) -> VTF {
        let mut vtf = VTF::default();

        let ver_major: u32 = bytes.pread(4).unwrap();
        let ver_minor: u32 = bytes.pread(8).unwrap();
        vtf.version = format!("{}.{}", ver_major, ver_minor)
            .parse::<f32>()
            .unwrap();
        // let header_size: u32 = bytes.pread(12).unwrap();
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
                let pos = 80 + i as usize * 8;
                let end = 80 + (i as usize + 1) * 8;
                let tag = (
                    bytes.pread::<u8>(pos).unwrap(),
                    bytes.pread::<u8>(pos + 1).unwrap(),
                    bytes.pread::<u8>(pos + 2).unwrap(),
                );
                let flags = bytes.pread::<u8>(pos + 3).unwrap();
                let offset = bytes.pread::<u32>(pos + 4).unwrap();
                let mut last: usize = 0;
                if num_resources - 1 != i {
                    last = bytes.pread::<u32>(end + 4).unwrap() as usize;
                } else {
                    last = bytes.len();
                }

                if tag == (0x01, 0x00, 0x00) || tag == (0x30, 0x00, 0x00) {
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
                        ABGR8888 | ARGB8888 | RGBA8888 => 4,
                        BGR888 | RGB888 => 3,
                        DXT1 | DXT3 | DXT5 => 4,
                        _ => 0,
                    };
                    let mut buffer: Vec<(u8, u8, u8, u8)> = vec![];
                    let mut j = 0;
                    let limit = (height as usize) * (width as usize);

                    if format == DXT1 || format == DXT3 || format == DXT5 {
                        let version = match format {
                            DXT1 => 1,
                            DXT3 => 3,
                            DXT5 => 5,
                            _ => 0,
                        };
                        let readpos = j * read_length + offset as usize;
                        let data = &bytes[readpos..last as usize];
                        let decompressed =
                            dxt::decode_dxt(version, data, width as usize, height as usize);
                        let decompressed = decompressed.as_slice();
                        while j * read_length < limit {
                            let readpos = j * read_length;
                            let color = &decompressed[readpos..(readpos + read_length)];
                            buffer.push(get_color(&format, color));
                            j += 1;
                        }
                    } else {
                        while j < limit {
                            let readpos = j * read_length + offset as usize;
                            let color = &bytes[readpos..(readpos + read_length)];
                            buffer.push(get_color(&format, color));
                            j += 1;
                        }
                    }

                    vtf.image_data.push(ImageData {
                        is_hi_res: hi_res,
                        data: buffer,
                    });
                } else {
                    match tag {
                        _ => vtf.resource_entries.push(ResourceEntry {
                            tag: tag,
                            flags: flags,
                            offset: offset,
                        }),
                    }
                }

                i += 1;
            }
        }

        vtf
    }
}
