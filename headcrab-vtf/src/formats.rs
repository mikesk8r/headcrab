// https://developer.valvesoftware.com/wiki/VTF_(Valve_Texture_Format)#Image_format
use scroll::Pread;

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

pub fn get_format_from_id(format: i32) -> ImageDataFormat {
    use ImageDataFormat::*;

    return match format {
        -1 => Unknown,
        0 => RGBA8888,
        1 => ABGR8888,
        2 => RGB888,
        3 => BGR888,
        9 => RGB888Bluescreen,
        10 => BGR888Bluescreen,
        11 => ARGB8888,
        13 => DXT1,
        14 => DXT3,
        15 => DXT5,
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
        // we don't really want to use this function with dxt...
        DXT1 | DXT3 | DXT5 => (0, 0, 0, 0),
    };
}
