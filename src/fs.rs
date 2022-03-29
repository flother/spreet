use std::fs::{read, read_dir, DirEntry};
use std::path::{Path, PathBuf};

use usvg::{Options, Tree};

use crate::error::Error;

/// Returns `true` if `entry`'s file name starts with `.`, `false` otherwise.
fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

/// Returns `true` if `entry` is a file with the extension `.svg`, `false` otherwise.
fn is_svg_file(entry: &DirEntry) -> bool {
    entry.path().is_file()
        && entry
            .path()
            .extension()
            .map(|s| s == "svg")
            .unwrap_or(false)
}

/// Returns `true` if `entry` is an SVG image and isn't hidden.
pub fn is_useful_input(entry: &DirEntry) -> bool {
    !is_hidden(entry) && is_svg_file(entry)
}

/// Returns a vector of file paths matching all SVGs within the given directory.
///
/// This does not recurse into sub-directories; they are silently ignored. It ignores hidden files
/// (files whose names begin with `.`) but it does follow symlinks.
pub fn get_svg_input_paths(path: &Path) -> Vec<PathBuf> {
    read_dir(&path)
        .unwrap()
        .flatten()
        .filter(is_useful_input)
        .map(|entry| entry.path())
        .collect()
}

// Load an SVG image from a file path.
pub fn load_svg(path: &Path) -> Result<Tree, Error> {
    Ok(Tree::from_data(
        &read(&path)?,
        &Options::default().to_ref(),
    )?)
}
