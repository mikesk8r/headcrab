pub fn decode_dxt(version: usize, data: &[u8], width: usize, height: usize) -> Vec<u8> {
    let mut output: Vec<u8> = vec![0; width * height * 4];

    match version {
        1 => {
            texpresso::Format::Bc1.decompress(data, width, height, output.as_mut_slice());
        }
        3 => {
            texpresso::Format::Bc2.decompress(data, width, height, output.as_mut_slice());
        }
        5 => {
            texpresso::Format::Bc3.decompress(data, width, height, output.as_mut_slice());
        }
        _ => {}
    }

    output
}
