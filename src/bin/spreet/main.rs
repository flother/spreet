use std::collections::BTreeMap;
use std::num::NonZero;

use clap::Parser;
use spreet::{get_svg_input_paths, load_svg, sprite_name, Optlevel, Sprite, Spritesheet};

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
                let sprite = if args.sdf {
                    Sprite::new_sdf(tree, pixel_ratio).expect("failed to load an SDF sprite")
                } else {
                    Sprite::new(tree, pixel_ratio).expect("failed to load a sprite")
                };
                if let Ok(name) = sprite_name(svg_path, args.input.as_path()) {
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
        .collect::<BTreeMap<String, Sprite>>();

    if sprites.is_empty() {
        eprintln!("Error: no valid SVGs found in {:?}", args.input);
        std::process::exit(exitcode::NOINPUT);
    }

    let mut spritesheet_builder = Spritesheet::build();
    spritesheet_builder.sprites(sprites);
    spritesheet_builder.spacing(args.spacing);
    if args.unique {
        spritesheet_builder.make_unique();
    };
    if args.sdf {
        spritesheet_builder.make_sdf();
    };

    // Generate sprite sheet
    let Some(spritesheet) = spritesheet_builder.generate() else {
        eprintln!("Error: could not pack the sprites within an area fifty times their size.");
        std::process::exit(exitcode::DATAERR);
    };

    let optlevel = match (args.oxipng, args.zopfli) {
        (None, None) => Optlevel::default(),
        (Some(level), None) => Optlevel::Oxipng { level },
        (None, Some(iterations)) => Optlevel::Zopfli {
            iterations: NonZero::new(iterations).unwrap(),
        },
        (Some(_), Some(_)) => unreachable!(),
    };

    // Save the bitmapped spritesheet to a local PNG.
    let file_prefix = args.output;
    let spritesheet_path = format!("{file_prefix}.png");
    if let Err(e) = spritesheet.save_spritesheet_at(&spritesheet_path, optlevel) {
        eprintln!("Error: could not save spritesheet to {spritesheet_path} ({e})");
        std::process::exit(exitcode::IOERR);
    };

    // Save the index file to a local JSON file with the same name as the spritesheet.
    if let Err(e) = spritesheet.save_index(&file_prefix, args.minify_index_file) {
        eprintln!("Error: could not save sprite index to {file_prefix} ({e})");
        std::process::exit(exitcode::IOERR);
    };
}
