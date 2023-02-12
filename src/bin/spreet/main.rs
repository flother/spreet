use std::collections::BTreeMap;

use clap::Parser;
use rayon::prelude::*;
use resvg::tiny_skia::Pixmap;

use spreet::fs::{get_svg_input_paths, load_svg};
use spreet::sprite;

mod cli;

fn main() {
    let args = cli::Cli::parse();

    // The ratio between the pixels in an SVG image and the pixels in the resulting PNG sprite. A
    // value of 2 means the PNGs will be double the size of the SVG images.
    let pixel_ratio = if args.retina { 2 } else { args.ratio };

    // Collect the file paths for all SVG images in the input directory.
    let svg_paths = get_svg_input_paths(&args.input, args.recursive);
    if svg_paths.is_empty() {
        eprintln!("Error: no SVGs found in {:?}", &args.input);
        std::process::exit(exitcode::NOINPUT);
    }

    // Read from all the input SVG files, convert them into bitmaps at the correct pixel ratio, and
    // store them in a map. The keys are the SVG filenames without the `.svg` extension. The
    // bitmapped SVGs will be added to the spritesheet, and the keys will be used as the unique
    // sprite ids in the JSON index file.
    let sprites = BTreeMap::from_iter(
        svg_paths
            .par_iter()
            .map(|svg_path| match load_svg(svg_path) {
                Ok(svg) => (
                    sprite::sprite_name(svg_path, args.input.as_path()),
                    sprite::generate_pixmap_from_svg(svg, pixel_ratio).unwrap(),
                ),
                Err(_) => {
                    eprintln!("{:?}: not a valid SVG image", svg_path);
                    std::process::exit(exitcode::DATAERR);
                }
            })
            .collect::<Vec<(String, Pixmap)>>(),
    );
    if sprites.is_empty() {
        eprintln!("Error: no valid SVGs found in {:?}", &args.input);
        std::process::exit(exitcode::NOINPUT);
    }

    let mut spritesheet_builder = sprite::Spritesheet::build();
    spritesheet_builder
        .sprites(sprites)
        .pixel_ratio(pixel_ratio);
    if args.unique {
        spritesheet_builder.make_unique();
    };
    // Save the spritesheet and index file if the sprites could be packed successfully, or exit with
    // an error code if not.
    if let Some(spritesheet) = spritesheet_builder.generate() {
        let spritesheet_path = format!("{}.png", args.output);
        // Save the bitmapped spritesheet to a local PNG.
        if let Err(e) = spritesheet.save_spritesheet(&spritesheet_path) {
            eprintln!(
                "Error: could not save spritesheet to {} ({})",
                spritesheet_path, e
            );
            std::process::exit(exitcode::IOERR);
        };
        // Save the index file to a local JSON file with the same name as the spritesheet.
        if let Err(e) = spritesheet.save_index(&args.output, args.minify_index_file) {
            eprintln!(
                "Error: could not save sprite index to {} ({})",
                args.output, e
            );
            std::process::exit(exitcode::IOERR);
        };
    } else {
        eprintln!("Error: could not pack the sprites within an area fifty times their size.");
        std::process::exit(exitcode::DATAERR);
    };
}
