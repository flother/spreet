use std::collections::BTreeMap;

use clap::Parser;
use spreet::fs::{get_svg_input_paths, load_svg};
use spreet::sprite;

mod cli;

fn main() {
    let args = cli::Cli::parse();

    // The ratio between the pixels in an SVG image and the pixels in the resulting PNG sprite. A
    // value of 2 means the PNGs will be double the size of the SVG images.
    let pixel_ratio = if args.retina { 2 } else { args.ratio };

    // Collect the file paths for all SVG images in the input directory.
    // Read from all the input SVG files, convert them into bitmaps at the correct pixel ratio, and
    // store them in a map. The keys are the SVG filenames without the `.svg` extension. The
    // bitmapped SVGs will be added to the spritesheet, and the keys will be used as the unique
    // sprite ids in the JSON index file.
    let Ok(input_paths) = get_svg_input_paths(&args.input, args.recursive) else {
        eprintln!("Error: no valid SVGs found in {:?}", args.input);
        std::process::exit(exitcode::NOINPUT);
    };
    let sprites = input_paths
        .iter()
        .map(|svg_path| {
            if let Ok(tree) = load_svg(svg_path) {
                let sprite = sprite::Sprite { tree, pixel_ratio };
                if let Ok(name) = sprite::sprite_name(svg_path, args.input.as_path()) {
                    (name, sprite)
                } else {
                    eprintln!("Error: cannot make a valid sprite name from {svg_path:?}");
                    std::process::exit(exitcode::DATAERR);
                }
            } else {
                eprintln!("{svg_path:?}: not a valid SVG image");
                std::process::exit(exitcode::DATAERR);
            }
        })
        .collect::<BTreeMap<String, sprite::Sprite>>();

    if sprites.is_empty() {
        eprintln!("Error: no valid SVGs found in {:?}", args.input);
        std::process::exit(exitcode::NOINPUT);
    }

    let mut spritesheet_builder = sprite::Spritesheet::build();
    spritesheet_builder.sprites(sprites);
    if args.unique {
        spritesheet_builder.make_unique();
    };

    // Generate sprite sheet
    let Some(spritesheet) = spritesheet_builder.generate() else {
        eprintln!("Error: could not pack the sprites within an area fifty times their size.");
        std::process::exit(exitcode::DATAERR);
    };

    // Save the bitmapped spritesheet to a local PNG.
    let file_prefix = args.output;
    let spritesheet_path = format!("{file_prefix}.png");
    if let Err(e) = spritesheet.save_spritesheet(&spritesheet_path) {
        eprintln!("Error: could not save spritesheet to {spritesheet_path} ({e})");
        std::process::exit(exitcode::IOERR);
    };

    // Save the index file to a local JSON file with the same name as the spritesheet.
    if let Err(e) = spritesheet.save_index(&file_prefix, args.minify_index_file) {
        eprintln!("Error: could not save sprite index to {file_prefix} ({e})");
        std::process::exit(exitcode::IOERR);
    };
}
