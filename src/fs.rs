use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;

use png::EncodingError;
use serde_json::json;
use tiny_skia::Pixmap;
use walkdir::DirEntry;

use crate::sprite::SpriteDescription;

/// Returns `true` if `entry`'s file name starts with `.`, `false` otherwise.
fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

/// Returns `true` if `entry` is a directory in the filesystem, `false` otherwise.
fn is_dir(entry: &DirEntry) -> bool {
    entry.path().is_dir()
}

/// Returns `true` if `entry` is a file with the extension `.svg`, `false` otherwise.
fn is_svg(entry: &DirEntry) -> bool {
    entry.path().is_file()
        && entry
            .path()
            .extension()
            .map(|s| s == "svg")
            .unwrap_or(false)
}

/// Returns `true` if `entry` is either a directory or an SVG image, and isn't hidden.
pub fn is_interesting_input(entry: &DirEntry) -> bool {
    !is_hidden(entry) && (is_dir(entry) || is_svg(entry))
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
    file_name_prefix: &String,
    sprite_index: BTreeMap<&String, SpriteDescription>,
) -> std::io::Result<()> {
    let mut file = File::create(format!("{}.json", file_name_prefix))?;
    write!(file, "{}", json!(sprite_index))?; // TODO: Save pretty-printed output.
    Ok(())
}

/// Saves the spritesheet to a local file names `file_name_prefix` + ".png".
///
/// A spritesheet, called an [image file] in the Mapbox Style Specification, is a PNG image
/// containing all the individual sprite images.
///
/// The spritesheet will match an index file that can be saved with [`save_sprite_index_file`].
///
/// [image file]: https://docs.mapbox.com/mapbox-gl-js/style-spec/sprite/#image-file
pub fn save_spritesheet(
    file_name_prefix: &String,
    spritesheet: Pixmap,
) -> Result<(), EncodingError> {
    spritesheet.save_png(format!("{}.png", file_name_prefix))?;
    Ok(())
}
