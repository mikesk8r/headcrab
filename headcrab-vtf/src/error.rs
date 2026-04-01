#[derive(Debug)]
pub enum Error {
    InvalidHeader,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Error::InvalidHeader => "invalid header (does not start with `VPK\\0`",
        })
    }
}
