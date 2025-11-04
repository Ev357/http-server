#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Path(std::path::StripPrefixError),
    InvalidMethod,
    InvalidPath,
    InvalidVersion,
    ProtocolError,
    NotFound(String),
    InternalError,
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IO(error)
    }
}

impl From<std::path::StripPrefixError> for Error {
    fn from(error: std::path::StripPrefixError) -> Self {
        Error::Path(error)
    }
}
