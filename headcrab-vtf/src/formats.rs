// https://developer.valvesoftware.com/wiki/VTF_(Valve_Texture_Format)#Image_format
use super::ImageDataFormat::{self};
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
        22 => UV88,
        _ => Unknown,
    };
}

pub fn get_color(format: &ImageDataFormat, bytes: &[u8]) -> (u8, u8, u8, u8) {
    use ImageDataFormat::*;

    return match format {
        Unknown => (0, 0, 0, 0),
        RGBA8888 => {
            let red: u8 = bytes.pread(0).unwrap();
            let green: u8 = bytes.pread(1).unwrap();
            let blue: u8 = bytes.pread(2).unwrap();
            let alpha: u8 = bytes.pread(3).unwrap();

            (red, green, blue, alpha)
        }
        ABGR8888 => {
            let red: u8 = bytes.pread(3).unwrap();
            let green: u8 = bytes.pread(2).unwrap();
            let blue: u8 = bytes.pread(1).unwrap();
            let alpha: u8 = bytes.pread(0).unwrap();

            (red, green, blue, alpha)
        }
        RGB888 => {
            let red: u8 = bytes.pread(0).unwrap();
            let green: u8 = bytes.pread(1).unwrap();
            let blue: u8 = bytes.pread(2).unwrap();

            (red, green, blue, 255)
        }
        BGR888 => {
            let red: u8 = bytes.pread(2).unwrap();
            let green: u8 = bytes.pread(1).unwrap();
            let blue: u8 = bytes.pread(0).unwrap();

            (red, green, blue, 255)
        }
        I8 => {
            let luminance: u8 = bytes.pread(0).unwrap();

            (luminance, luminance, luminance, 255)
        }
        IA88 => {
            let luminance: u8 = bytes.pread(0).unwrap();
            let alpha: u8 = bytes.pread(0).unwrap();

            (luminance, luminance, luminance, alpha)
        }
        A8 => {
            let alpha: u8 = bytes.pread(0).unwrap();

            (255, 255, 255, alpha)
        }
        RGB888Bluescreen => {
            let red: u8 = bytes.pread(0).unwrap();
            let green: u8 = bytes.pread(1).unwrap();
            let blue: u8 = bytes.pread(2).unwrap();

            if blue == 255 {
                if red + green == 0 {
                    return (0, 0, 255, 0);
                }
            }

            (red, green, blue, 255)
        }
        BGR888Bluescreen => {
            let red: u8 = bytes.pread(3).unwrap();
            let green: u8 = bytes.pread(2).unwrap();
            let blue: u8 = bytes.pread(1).unwrap();

            if blue == 255 {
                if red + green == 0 {
                    return (0, 0, 255, 0);
                }
            }

            (red, green, blue, 255)
        }
        ARGB8888 => {
            let red: u8 = bytes.pread(1).unwrap();
            let green: u8 = bytes.pread(2).unwrap();
            let blue: u8 = bytes.pread(3).unwrap();
            let alpha: u8 = bytes.pread(0).unwrap();

            (red, green, blue, alpha)
        }
        DXT1 | DXT3 | DXT5 => {
            let red: u8 = bytes.pread(1).unwrap();
            let green: u8 = bytes.pread(2).unwrap();
            let blue: u8 = bytes.pread(3).unwrap();
            let alpha: u8 = bytes.pread(0).unwrap();

            (red, green, blue, alpha)
        }
        UV88 => {
            let red: u8 = bytes.pread(0).unwrap();
            let green: u8 = bytes.pread(1).unwrap();

            (red, green, 0, 255)
        }
    };
}
