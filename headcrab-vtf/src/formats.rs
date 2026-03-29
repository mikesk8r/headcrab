// https://developer.valvesoftware.com/wiki/VTF_(Valve_Texture_Format)#Image_format
use super::{
    ColorChannel,
    ImageDataFormat::{self},
};
use scroll::Pread;

pub fn get_format_from_id(format: i32) -> ImageDataFormat {
    use ImageDataFormat::*;

    return match format {
        -1 => Unknown,
        0 => RGBA8888,
        1 => ABGR8888,
        2 => RGB888,
        3 => BGR888,
        5 => I8,
        6 => IA88,
        8 => A8,
        9 => RGB888Bluescreen,
        10 => BGR888Bluescreen,
        11 => ARGB8888,
        13 => DXT1,
        14 => DXT3,
        15 => DXT5,
        16 => BGRX8888,
        22 => UV88,
        23 => UVWQ8888,
        25 => RGBA16161616,
        26 => UVLX8888,
        _ => Unknown,
    };
}

pub fn get_color<T>(format: &ImageDataFormat, bytes: &[u8]) -> (T, T, T, T)
where
    T: ColorChannel + From<u8> + From<u16>,
{
    use ImageDataFormat::*;

    return match format {
        Unknown => (0u8.into(), 0u8.into(), 0u8.into(), 0u8.into()),
        RGBA8888 => {
            let red: u8 = bytes.pread(0).unwrap();
            let green: u8 = bytes.pread(1).unwrap();
            let blue: u8 = bytes.pread(2).unwrap();
            let alpha: u8 = bytes.pread(3).unwrap();

            (red.into(), green.into(), blue.into(), alpha.into())
        }
        ABGR8888 => {
            let red: u8 = bytes.pread(3).unwrap();
            let green: u8 = bytes.pread(2).unwrap();
            let blue: u8 = bytes.pread(1).unwrap();
            let alpha: u8 = bytes.pread(0).unwrap();

            (red.into(), green.into(), blue.into(), alpha.into())
        }
        RGB888 => {
            let red: u8 = bytes.pread(0).unwrap();
            let green: u8 = bytes.pread(1).unwrap();
            let blue: u8 = bytes.pread(2).unwrap();

            (red.into(), green.into(), blue.into(), 255u8.into())
        }
        BGR888 => {
            let red: u8 = bytes.pread(2).unwrap();
            let green: u8 = bytes.pread(1).unwrap();
            let blue: u8 = bytes.pread(0).unwrap();

            (red.into(), green.into(), blue.into(), 255u8.into())
        }
        I8 => {
            let luminance: u8 = bytes.pread(0).unwrap();

            (
                luminance.into(),
                luminance.into(),
                luminance.into(),
                255u8.into(),
            )
        }
        IA88 => {
            let luminance: u8 = bytes.pread(0).unwrap();
            let alpha: u8 = bytes.pread(1).unwrap();

            (
                luminance.into(),
                luminance.into(),
                luminance.into(),
                alpha.into(),
            )
        }
        A8 => {
            let alpha: u8 = bytes.pread(0).unwrap();

            (255u8.into(), 255u8.into(), 255u8.into(), alpha.into())
        }
        RGB888Bluescreen => {
            let red: u8 = bytes.pread(0).unwrap();
            let green: u8 = bytes.pread(1).unwrap();
            let blue: u8 = bytes.pread(2).unwrap();

            if blue == 255 {
                if red + green == 0 {
                    return (0u8.into(), 0u8.into(), 255u8.into(), 0u8.into());
                }
            }

            (red.into(), green.into(), blue.into(), 255u8.into())
        }
        BGR888Bluescreen => {
            let red: u8 = bytes.pread(3).unwrap();
            let green: u8 = bytes.pread(2).unwrap();
            let blue: u8 = bytes.pread(1).unwrap();

            if blue == 255 {
                if red + green == 0 {
                    return (0u8.into(), 0u8.into(), 255u8.into(), 0u8.into());
                }
            }

            (red.into(), green.into(), blue.into(), 255u8.into())
        }
        ARGB8888 => {
            let red: u8 = bytes.pread(1).unwrap();
            let green: u8 = bytes.pread(2).unwrap();
            let blue: u8 = bytes.pread(3).unwrap();
            let alpha: u8 = bytes.pread(0).unwrap();

            (red.into(), green.into(), blue.into(), alpha.into())
        }
        DXT1 | DXT3 | DXT5 => {
            let red: u8 = bytes.pread(1).unwrap();
            let green: u8 = bytes.pread(2).unwrap();
            let blue: u8 = bytes.pread(3).unwrap();
            let alpha: u8 = bytes.pread(0).unwrap();

            (red.into(), green.into(), blue.into(), alpha.into())
        }
        BGRX8888 => {
            let red: u8 = bytes.pread(2).unwrap();
            let green: u8 = bytes.pread(1).unwrap();
            let blue: u8 = bytes.pread(0).unwrap();

            (red.into(), green.into(), blue.into(), 255u8.into())
        }
        UV88 => {
            let red: u8 = bytes.pread(0).unwrap();
            let green: u8 = bytes.pread(1).unwrap();

            (red.into(), green.into(), 0u8.into(), 255u8.into())
        }
        UVWQ8888 | UVLX8888 => {
            let u: u8 = bytes.pread(0).unwrap();
            let v: u8 = bytes.pread(1).unwrap();
            let w_l: u8 = bytes.pread(2).unwrap();
            let q_x: u8 = bytes.pread(3).unwrap();

            (u.into(), v.into(), w_l.into(), q_x.into())
        }
        RGBA16161616 => {
            let red: u16 = bytes.pread(0).unwrap();
            let green: u16 = bytes.pread(1).unwrap();
            let blue: u16 = bytes.pread(2).unwrap();
            let alpha: u16 = bytes.pread(3).unwrap();

            (red.into(), green.into(), blue.into(), alpha.into())
        }
    };
}
