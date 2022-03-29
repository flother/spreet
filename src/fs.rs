use std::collections::BTreeMap;
use std::fs::{read, read_dir, DirEntry, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use oxipng::optimize_from_memory;
use tiny_skia::Pixmap;
use usvg::{Options, Tree};

use crate::error::Error;
use crate::sprite::SpriteDescription;

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

/// Saves the `sprite_index` to a local file named `file_name_prefix` + ".json".
///
/// An [index file] is defined in the Mapbox Style Specification as a JSON document containing a
/// description of each sprite within a spritesheet. It contains the width, height, x and y
/// positions, and pixel ratio of the sprite.
///
/// The index file will match a spritesheet that can be saved with [`save_spritesheet`].
///
/// [index file]: https://docs.mapbox.com/mapbox-gl-js/style-spec/sprite/#index-file
pub fn save_sprite_index_file(
    file_name_prefix: &str,
    sprite_index: BTreeMap<&String, SpriteDescription>,
) -> std::io::Result<()> {
    let mut file = File::create(format!("{}.json", file_name_prefix))?;
    let json_string = serde_json::to_string_pretty(&sprite_index)?;
    write!(file, "{}", json_string)?;
    Ok(())
}

/// Saves the spritesheet to a local file named `path`.
///
/// A spritesheet, called an [image file] in the Mapbox Style Specification, is a PNG image
/// containing all the individual sprite images. The `spritesheet` `Pixmap` is converted to an
/// in-memory PNG, optimised using the [`oxipng`] library, and saved to a local file.
///
/// The spritesheet will match an index file that can be saved with [`save_sprite_index_file`].
///
/// [image file]: https://docs.mapbox.com/mapbox-gl-js/style-spec/sprite/#image-file
/// [`oxipng`]: https://github.com/shssoichiro/oxipng
pub fn save_spritesheet(path: &str, spritesheet: Pixmap) -> Result<(), Error> {
    let spritesheet_png = spritesheet.encode_png()?;
    let spritesheet_png =
        optimize_from_memory(spritesheet_png.as_slice(), &oxipng::Options::default())?;
    std::fs::write(path, spritesheet_png)?;
    Ok(())
}
