use std::path::PathBuf;
use std::str::FromStr;

use clap::{ArgGroup, Parser};

/// Container for Spreet's command-line arguments.
#[derive(Parser)]
#[command(version, about)]
#[command(group(ArgGroup::new("pixel_ratio").args(&["ratio", "retina"])))]
pub struct Cli {
    /// A directory of SVGs to include in the spritesheet
    #[arg(value_parser = is_dir)]
    pub input: PathBuf,
    /// Name of the file in which to save the spritesheet
    pub output: String,
    /// Set the output pixel ratio
    #[arg(short, long, default_value_t = 1, value_parser = is_positive)]
    pub ratio: u8,
    /// Set the pixel ratio to 2 (equivalent to `--ratio=2`)
    #[arg(long)]
    pub retina: bool,
    /// Store only unique images in the spritesheet, and map them to multiple names
    #[arg(long)]
    pub unique: bool,
    /// Include images in sub-directories
    #[arg(long)]
    pub recursive: bool,
    /// Add pixel spacing between sprites
    #[arg(long, default_value_t = 0, value_parser = is_non_negative)]
    pub spacing: u8,
    /// Remove whitespace from the JSON index file
    #[arg(short, long)]
    pub minify_index_file: bool,
    #[arg(long)]
    /// Output only x, y, width, and height to the JSON index file
    pub simple_index_file: bool,
    /// Output a spritesheet using a signed distance field for each sprite
    #[arg(long)]
    pub sdf: bool,
}

/// Clap validator to ensure that a string is an existing directory.
fn is_dir(p: &str) -> Result<PathBuf, String> {
    if PathBuf::from(p).is_dir() {
        Ok(p.into())
    } else {
        Err(String::from("must be an existing directory"))
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

/// Clap validator to ensure that an unsigned integer parsed from a string is non-negative.
fn is_non_negative(s: &str) -> Result<u8, String> {
    u8::from_str(s)
        .map_err(|_| String::from("must be a non-negative number"))
        .and_then(|result| {
            // u8 is inherently non-negative, so we just need to validate parsing
            Ok(result)
        })
}
