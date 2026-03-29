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
    I8,
    IA88,
    A8,
    RGB888Bluescreen,
    BGR888Bluescreen,
    ARGB8888,
    DXT1,
    DXT3,
    DXT5,
    BGRX8888,
    /// Image data layout: [u, v, 0, 255]
    UV88,
    UVWQ8888,
    RGBA16161616,
    UVLX8888,
}

bitflags! {
    pub struct VTFFlags: u32 {
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

pub trait ColorChannel {}

impl ColorChannel for u8 {}
impl ColorChannel for u16 {}
impl ColorChannel for u32 {}
impl ColorChannel for u64 {}
impl ColorChannel for f32 {}
impl ColorChannel for f64 {}

/// Data is RGBA
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Texture<T>
where
    T: ColorChannel,
{
    pub data: Vec<Vec<Vec<T>>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourceEntry {
    pub tag: (u8, u8, u8),
    pub flags: u8,
    pub offset: u32,
    // TODO: maybe add field for data? not really sure
}

#[derive(Debug, Default, PartialEq)]
pub struct VTF<T>
where
    T: ColorChannel,
{
    /// (maj, min) -- for example (7, 6) for VTF 7.6
    pub version: (u8, u8),
    pub width: u16,
    pub height: u16,
    /// See VTFFlags
    pub flags: u32,
    pub frames: u16,
    pub first_frame: u16,
    pub reflectivity: (f32, f32, f32),
    pub bumpmap_scale: f32,
    pub resource_entries: Vec<ResourceEntry>,
    /// Always 0 when version < 7.2
    pub depth: u16,

    pub texture: Texture<T>,
    pub texture_format: ImageDataFormat,
    /// Highest resolution to lowest. Does not include original texture
    pub mipmaps: Vec<Texture<T>>,
    pub mipmap_count: u8,

    /// Data is RGBA
    pub thumbnail: Vec<u8>,
    pub thumbnail_format: ImageDataFormat,
    pub thumbnail_width: u8,
    pub thumbnail_height: u8,
}

impl<T> VTF<T>
where
    T: ColorChannel + Default + From<u8> + From<u16>,
{
    pub fn from_bytes(bytes: &[u8]) -> VTF<T> {
        let mut vtf: VTF<T> = VTF::default();

        let ver_major: u32 = bytes.pread(4).unwrap();
        let ver_minor: u32 = bytes.pread(8).unwrap();
        vtf.version = (ver_major as u8, ver_minor as u8);
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
        vtf.texture_format = get_format_from_id(bytes.pread(52).unwrap());
        vtf.mipmap_count = bytes.pread::<u8>(56).unwrap() - 1;
        vtf.thumbnail_format = get_format_from_id(bytes.pread(57).unwrap());
        vtf.thumbnail_width = bytes.pread(61).unwrap();
        vtf.thumbnail_height = bytes.pread(62).unwrap();
        if vtf.version.1 >= 2 {
            vtf.depth = bytes.pread(63).unwrap();
        }

        let mut skip: usize = header_size as usize;
        let mut thumbnail_skip: usize = 0;
        if vtf.version.1 >= 3 {
            let num_resources = bytes.pread::<u32>(68).unwrap();
            let mut j = 0;
            while j < num_resources {
                let pos = 80 + j as usize * 8;
                let tag = (
                    bytes.pread::<u8>(pos).unwrap(),
                    bytes.pread::<u8>(pos + 1).unwrap(),
                    bytes.pread::<u8>(pos + 2).unwrap(),
                );
                let flags = bytes.pread::<u8>(pos + 3).unwrap();
                let offset = bytes.pread::<u32>(pos + 4).unwrap();

                if tag == (0x30, 0x00, 0x00) {
                    skip = offset as usize;
                } else if tag == (0x01, 0x00, 0x00) {
                    thumbnail_skip = offset as usize;
                } else {
                    vtf.resource_entries.push(ResourceEntry {
                        tag: tag,
                        flags: flags,
                        offset: offset,
                    });
                }

                j += 1;
            }
        }

        let thumbnail_compressed = &bytes[thumbnail_skip as usize..thumbnail_skip as usize + 256];
        vtf.thumbnail = dxt::decode_dxt(ImageDataFormat::DXT1, thumbnail_compressed, 16, 16);

        use ImageDataFormat::*;
        let read_length = match vtf.texture_format {
            A8 | I8 => 1,
            IA88 | UV88 => 2,
            BGR888 | RGB888 | BGR888Bluescreen | RGB888Bluescreen => 3,
            _ => 4,
        };

        let mut i = 0;
        while i < (vtf.mipmap_count + 1) {
            let mut j = 0;
            let (current_width, current_height, current_size);
            current_width = vtf.width as usize / 2_usize.pow(i.into());
            current_height = vtf.height as usize / 2_usize.pow(i.into());
            current_size = current_width * current_height;
            let mut frames: Vec<Vec<Vec<T>>> = vec![vec![vec![]]];

            while j < vtf.frames {
                let mut k = 0;
                let mut slices: Vec<Vec<T>> = vec![vec![]];

                while k < vtf.depth {
                    let mut pixels: Vec<T> = vec![];

                    if vtf.texture_format == DXT1
                        || vtf.texture_format == DXT3
                        || vtf.texture_format == DXT5
                    {
                        let compressed = &bytes[skip..skip + current_size];
                        let decompressed = dxt::decode_dxt(
                            vtf.texture_format,
                            compressed,
                            current_width,
                            current_height,
                        );

                        let mut l = 0;
                        while l < decompressed.len() - 2 {
                            let color: (T, T, T, T) =
                                get_color(&vtf.texture_format, &decompressed[l..l + read_length]);
                            pixels.push(color.0);
                            pixels.push(color.1);
                            pixels.push(color.2);
                            pixels.push(color.3);
                            l += 4;
                        }
                    } else {
                        let mut l = 0;
                        while l < (current_size * read_length) - (read_length - 2) {
                            let color: (T, T, T, T) = get_color(
                                &vtf.texture_format,
                                &bytes[l + skip..l + read_length + skip],
                            );
                            pixels.push(color.0);
                            pixels.push(color.1);
                            pixels.push(color.2);
                            pixels.push(color.3);
                            l += read_length;
                        }
                    }

                    slices.push(pixels);

                    skip += current_size;
                    k += 1;
                }

                frames.push(slices);
                j += 1;
            }

            if i == vtf.mipmap_count {
                vtf.texture = Texture { data: frames };
            } else {
                vtf.mipmaps.push(Texture { data: frames });
            }

            i += 1;
        }
        vtf.mipmaps.reverse();

        vtf
    }
}
