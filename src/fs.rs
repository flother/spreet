use std::fs::{read, read_dir, DirEntry};
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};

use resvg::usvg::fontdb::Database;
use resvg::usvg::{Options, Tree};

use crate::error::SpreetResult;

/// Returns `true` if `entry`'s file name starts with `.`, `false` otherwise.
fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map_or(false, |s| s.starts_with('.'))
}

/// Returns `true` if `entry` is a file with the extension `.svg`, `false` otherwise.
fn is_svg_file(entry: &DirEntry) -> bool {
    entry.path().is_file() && entry.path().extension().map_or(false, |s| s == "svg")
}

/// Returns `true` if `entry` is an SVG image and isn't hidden.
fn is_useful_input(entry: &DirEntry) -> bool {
    !is_hidden(entry) && is_svg_file(entry)
}

/// Returns a vector of file paths matching all SVGs within the given directory.
///
/// It ignores hidden files (files whose names begin with `.`) but it does follow symlinks. If
/// `recursive` is `true` it will also return file paths in sub-directories.
///
/// # Errors
///
/// This function will return an error if Rust's underlying [`read_dir`] returns an error.
pub fn get_svg_input_paths<P: AsRef<Path>>(path: P, recursive: bool) -> SpreetResult<Vec<PathBuf>> {
    Ok(read_dir(path)?
        .filter_map(|entry| {
            if let Ok(entry) = entry {
                let path_buf = entry.path();
                if recursive && path_buf.is_dir() {
                    get_svg_input_paths(path_buf, true).ok()
                } else if is_useful_input(&entry) {
                    Some(vec![path_buf])
                } else {
                    None
                }
            } else {
                None
            }
        })
        .flatten()
        .collect())
}

/// Load an SVG image from a file path.
pub fn load_svg<P: AsRef<Path>>(path: P) -> SpreetResult<Tree> {
    static FONTDB: OnceLock<Arc<Database>> = OnceLock::new();
    let fontdb = FONTDB
        .get_or_init(|| {
            let mut db = Database::new();
            db.load_system_fonts();
            Arc::new(db)
        })
        .clone();

    // The resources directory needs to be the same location as the SVG file itself, so that any
    // embedded resources (like PNGs in <image> elements) that use relative URLs can be resolved
    // correctly.
    let resources_dir = std::fs::canonicalize(&path)
        .ok()
        .and_then(|p| p.parent().map(Path::to_path_buf));
    let options = Options {
        resources_dir,
        fontdb,
        ..Options::default()
    };

    Ok(Tree::from_data(&read(path)?, &options)?)
}
