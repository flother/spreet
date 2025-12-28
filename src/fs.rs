use std::borrow::Cow;
use std::fs::{read, read_dir, DirEntry};
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};

use resvg::usvg::fontdb::Database;
use resvg::usvg::{decompress_svgz, roxmltree, Error as UsvgError, Options, Tree};

use crate::error::SpreetResult;

/// Returns `true` if `entry`'s file name starts with `.`, `false` otherwise.
fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .is_some_and(|s| s.starts_with('.'))
}

/// Returns `true` if `entry` is a file with the extension `.svg` or `.svgz`, `false` otherwise.
fn is_svg_file(entry: &DirEntry) -> bool {
    entry.path().is_file()
        && entry
            .path()
            .extension()
            .is_some_and(|s| s == "svg" || s == "svgz")
}

/// Returns `true` if `entry` is an SVG image and isn't hidden.
fn is_useful_input(entry: &DirEntry) -> bool {
    !is_hidden(entry) && is_svg_file(entry)
}

/// Returns a vector of file paths matching all SVG and SVGZ files within the given directory.
///
/// It ignores hidden files (files whose names begin with `.`) but it does follow symlinks. If
/// `recursive` is `true` it will also return file paths in sub-directories.
///
/// # Errors
///
/// This function will return an error if Rust's underlying [`read_dir`] returns an error.
pub fn get_svg_input_paths<P: AsRef<Path>>(path: P, recursive: bool) -> SpreetResult<Vec<PathBuf>> {
    let mut results = Vec::new();
    for entry in read_dir(path)? {
        let entry = entry?;
        let path_buf = entry.path();
        if recursive && path_buf.is_dir() {
            let nested = get_svg_input_paths(path_buf, true)?;
            results.extend(nested);
        } else if is_useful_input(&entry) {
            results.push(path_buf);
        }
    }
    Ok(results)
}

/// Load an SVG image from a file path.
pub fn load_svg<P: AsRef<Path>>(path: P) -> SpreetResult<Tree> {
    static SYSTEM_FONTDB: OnceLock<Arc<Database>> = OnceLock::new();
    static EMPTY_FONTDB: OnceLock<Arc<Database>> = OnceLock::new();

    let path = path.as_ref();
    let data = read(path)?;
    let text = svg_data_to_text(&data)?;
    let xml_opt = roxmltree::ParsingOptions {
        allow_dtd: true,
        ..Default::default()
    };
    let doc = roxmltree::Document::parse_with_options(&text, xml_opt).map_err(UsvgError::from)?;
    // Font database initialisation can be expensive, so only load system fonts if an SVG includes a
    // text element.
    let fontdb = if svg_has_text_nodes(&doc) {
        SYSTEM_FONTDB
            .get_or_init(|| {
                let mut db = Database::new();
                db.load_system_fonts();
                Arc::new(db)
            })
            .clone()
    } else {
        EMPTY_FONTDB
            .get_or_init(|| Arc::new(Database::new()))
            .clone()
    };

    // The resources directory needs to be the same location as the SVG file itself, so that any
    // embedded resources (like PNGs in <image> elements) that use relative URLs can be resolved
    // correctly.
    let resources_dir = std::fs::canonicalize(path)
        .ok()
        .and_then(|p| p.parent().map(Path::to_path_buf));
    let options = Options {
        resources_dir,
        fontdb,
        ..Options::default()
    };

    Ok(Tree::from_xmltree(&doc, &options)?)
}

/// Returns `true` if the SVG document contains any `<text>` nodes, `false` otherwise.
fn svg_has_text_nodes(doc: &roxmltree::Document) -> bool {
    doc.descendants().any(|n| n.has_tag_name("text"))
}

/// Convert SVG data (which may be compressed as SVGZ) into a UTF-8 string.
fn svg_data_to_text(data: &[u8]) -> Result<Cow<'_, str>, UsvgError> {
    if data.starts_with(&[0x1f, 0x8b]) {
        let data = decompress_svgz(data)?;
        let text = String::from_utf8(data).map_err(|_| UsvgError::NotAnUtf8Str)?;
        Ok(Cow::Owned(text))
    } else {
        let text = std::str::from_utf8(data).map_err(|_| UsvgError::NotAnUtf8Str)?;
        Ok(Cow::Borrowed(text))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_fs::prelude::*;
    #[cfg(unix)]
    use std::os::unix::fs::PermissionsExt;

    fn entry_for(temp: &assert_fs::TempDir, name: &str) -> DirEntry {
        std::fs::read_dir(temp.path())
            .unwrap()
            .map(|entry| entry.unwrap())
            .find(|entry| entry.file_name().to_str() == Some(name))
            .unwrap()
    }

    #[test]
    fn is_svg_file_accepts_svg() {
        let tmp_dir = assert_fs::TempDir::new().unwrap();
        tmp_dir.child("icon.svg").touch().unwrap();
        let svg_entry = entry_for(&tmp_dir, "icon.svg");
        assert!(is_svg_file(&svg_entry));
    }

    #[test]
    fn is_svg_file_accepts_svgz() {
        let tmp_dir = assert_fs::TempDir::new().unwrap();
        tmp_dir.child("icon.svgz").touch().unwrap();
        let svgz_entry = entry_for(&tmp_dir, "icon.svgz");
        assert!(is_svg_file(&svgz_entry));
    }

    #[test]
    fn is_svg_file_rejects_other_extensions() {
        let tmp_dir = assert_fs::TempDir::new().unwrap();
        tmp_dir.child("icon.png").touch().unwrap();
        let png_entry = entry_for(&tmp_dir, "icon.png");
        assert!(!is_svg_file(&png_entry));
    }

    #[test]
    fn is_svg_file_rejects_directories() {
        let tmp_dir = assert_fs::TempDir::new().unwrap();
        tmp_dir.child("icons.svg").create_dir_all().unwrap();
        let dir_entry = entry_for(&tmp_dir, "icons.svg");
        assert!(!is_svg_file(&dir_entry));
    }

    #[cfg(unix)]
    #[test]
    fn get_svg_input_paths_returns_error_on_unreadable_directory() {
        let tmp_dir = assert_fs::TempDir::new().unwrap();
        let restricted = tmp_dir.child("no-access");
        restricted.create_dir_all().unwrap();
        std::fs::set_permissions(restricted.path(), std::fs::Permissions::from_mode(0o000))
            .unwrap();

        let result = get_svg_input_paths(tmp_dir.path(), true);

        std::fs::set_permissions(restricted.path(), std::fs::Permissions::from_mode(0o700))
            .unwrap();
        assert!(result.is_err());
    }
}
