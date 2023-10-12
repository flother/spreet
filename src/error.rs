use std::io;
use std::path::PathBuf;

use oxipng::PngError;
use thiserror::Error;

pub type SpreetResult<T> = Result<T, Error>;

/// Possible errors encountered during execution.
#[derive(Debug, Error)]
pub enum Error {
    #[error("i/o error: {0}")]
    IoError(#[from] io::Error),
    #[error("Incorrect path {}", .0.display())]
    PathError(PathBuf),
    #[error("PNG encoding error: {0}")]
    PngError(#[from] png::EncodingError),
    #[error("Oxipng error: {0}")]
    OxiPngError(#[from] PngError),
    #[error("SVG error: {0}")]
    SvgError(#[from] resvg::usvg::Error),
}
