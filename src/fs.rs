use walkdir::DirEntry;

/// Returns `true` if `entry`'s file name starts with `.`, `false` otherwise.
fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
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
