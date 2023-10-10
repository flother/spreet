use std::fmt;

/// Possible errors encountered during execution.
#[derive(Debug, PartialEq)]
pub enum Error {
    IoError,
    PngError,
    SvgError,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::IoError => write!(f, "i/o error"),
            Error::PngError => write!(f, "PNG encoding error"),
            Error::SvgError => write!(f, "SVG error"),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {
        Error::IoError
    }
}

impl From<std::path::StripPrefixError> for Error {
    fn from(_: std::path::StripPrefixError) -> Self {
        Error::IoError
    }
}

impl From<resvg::usvg::Error> for Error {
    fn from(_: resvg::usvg::Error) -> Self {
        Error::SvgError
    }
}

impl From<png::EncodingError> for Error {
    fn from(_: png::EncodingError) -> Self {
        Error::PngError
    }
}

impl From<oxipng::PngError> for Error {
    fn from(_: oxipng::PngError) -> Self {
        Error::PngError
    }
}
