use std::path::PathBuf;
use std::str::FromStr;

use clap::{ArgGroup, Parser};

/// Container for Spreet's command-line arguments.
#[derive(Parser)]
#[clap(version, about)]
#[clap(group(ArgGroup::new("pixel_ratio").args(&["ratio", "retina"])))]
pub struct Cli {
    /// A directory of SVGs to include in the spritesheet
    #[clap(value_parser = is_dir)]
    pub input: PathBuf,
    /// Name of the file in which to save the spritesheet
    pub output: String,
    /// Set the output pixel ratio
    #[clap(short, long, default_value_t = 1, value_parser = is_positive)]
    pub ratio: u8,
    /// Set the pixel ratio to 2 (equivalent to `--ratio=2`)
    #[clap(long)]
    pub retina: bool,
    /// Store only unique images in the spritesheet, and map them to multiple names
    #[clap(long)]
    pub unique: bool,
    /// Remove whitespace from the JSON index file
    #[clap(short, long)]
    pub minify_index_file: bool,
}

/// Clap validator to ensure that a string is an existing directory.
fn is_dir(p: &str) -> Result<PathBuf, String> {
    match PathBuf::from(p).is_dir() {
        true => Ok(p.into()),
        false => Err(String::from("must be an existing directory")),
    }
}

/// Clap validator to ensure that an unsigned integer parsed from a string is greater than zero.
fn is_positive(s: &str) -> Result<u8, String> {
    u8::from_str(s)
        .map_err(|e| e.to_string())
        .and_then(|result| match result {
            i if i > 0 => Ok(result),
            _ => Err(String::from("must be greater than one")),
        })
}
