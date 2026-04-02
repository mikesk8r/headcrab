#[derive(Debug)]
pub enum Error {
    CannotFindFile,
    InvalidHeader,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Error::CannotFindFile => "cannot find or read the file specified. make sure you used `path/to/yourvpk_dir.vpk`!",
            Error::InvalidHeader => "invalid header (does not start with `0x55aa1234`",
        })
    }
}
