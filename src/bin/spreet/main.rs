use std::collections::BTreeMap;
use std::path::PathBuf;

use clap::Parser;
use serde_json::json;
use spreet::{get_png_input_paths, get_svg_input_paths, load_svg, sprite_name, Sprite, Spritesheet};

mod cli;

fn main() {
    let args = cli::Cli::parse();

    if args.png_pair {
        run_png_pair(&args);
        return;
    }

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

    generate_and_save(
        sprites,
        args.unique,
        args.sdf,
        args.spacing,
        args.minify_index_file,
        &args.output,
    );
}

/// Build a spritesheet from the given sprites.
fn build_spritesheet(
    sprites: BTreeMap<String, Sprite>,
    unique: bool,
    sdf: bool,
    spacing: u8,
) -> Spritesheet {
    let mut spritesheet_builder = Spritesheet::build().sprites(sprites).spacing(spacing);
    if unique {
        spritesheet_builder = spritesheet_builder.make_unique();
    }
    if sdf {
        spritesheet_builder = spritesheet_builder.make_sdf();
    }

    let Some(spritesheet) = spritesheet_builder.generate() else {
        eprintln!("Error: could not pack the sprites within an area fifty times their size.");
        std::process::exit(exitcode::DATAERR);
    };
    spritesheet
}

/// Save the spritesheet PNG and its (implicitly-serialized) index to `{file_prefix}.png` /
/// `{file_prefix}.json`.
fn generate_and_save(
    sprites: BTreeMap<String, Sprite>,
    unique: bool,
    sdf: bool,
    spacing: u8,
    minify_index_file: bool,
    file_prefix: &str,
) {
    let spritesheet = build_spritesheet(sprites, unique, sdf, spacing);

    // Save the bitmapped spritesheet to a local PNG.
    let spritesheet_path = format!("{file_prefix}.png");
    if let Err(e) = spritesheet.save_spritesheet(&spritesheet_path) {
        eprintln!("Error: could not save spritesheet to {spritesheet_path} ({e})");
        std::process::exit(exitcode::IOERR);
    };

    // Save the index file to a local JSON file with the same name as the spritesheet.
    if let Err(e) = spritesheet.save_index(file_prefix, minify_index_file) {
        eprintln!("Error: could not save sprite index to {file_prefix} ({e})");
        std::process::exit(exitcode::IOERR);
    };
}

/// Run in `--png-pair` mode: build spritesheets from paired PNG icons, mirroring Mapbox's
/// `build-sprite.js`.
///
/// The input directory is scanned for PNG files. Files are paired by their names with any `@2x`
/// suffix stripped: `xxx.png` pairs with `xxx@2x.png`. Two spritesheets are generated --- a 1x
/// spritesheet from the regular images (falling back to retina images when a regular one is
/// missing, mirroring `build-sprite.js:36-41`), and a 2x spritesheet from the retina images
/// (falling back to regular images). Each pair is written as `<OUTPUT>.png`/`<OUTPUT>.json` and
/// `<OUTPUT>@2x.png`/`<OUTPUT>@2x.json`.
///
/// Each sprite's `pixelRatio` in the index file is 2 when the source file name contains `@2x`
/// (even for a sprite in the 1x spritesheet that fell back to a retina image), and each sprite's
/// `sdf` flag is true when the source file path contains `sdf` --- both mirroring the per-file
/// detection in `build-sprite.js:58-59,70-72`.
fn run_png_pair(args: &cli::Cli) {
    // Collect the file paths for all PNG images in the input directory.
    let Ok(input_paths) = get_png_input_paths(&args.input, args.recursive) else {
        eprintln!("Error: no valid PNGs found in {:?}", args.input);
        std::process::exit(exitcode::NOINPUT);
    };

    // Pair regular and retina images by name (file name without the `.png` extension and any
    // `@2x` suffix), mirroring build-sprite.js:24-31.
    let mut pairs: BTreeMap<String, (Option<PathBuf>, Option<PathBuf>)> = BTreeMap::new();
    for path in input_paths {
        let Some(file_name) = path.file_name().map(|f| f.to_string_lossy().into_owned()) else {
            continue;
        };
        let retina = file_name.contains("@2x");
        let name = file_name
            .strip_suffix(".png")
            .unwrap_or(&file_name)
            .replace("@2x", "");
        let entry = pairs.entry(name).or_default();
        if retina {
            entry.1 = Some(path);
        } else {
            entry.0 = Some(path);
        }
    }

    if pairs.is_empty() {
        eprintln!("Error: no valid PNGs found in {:?}", args.input);
        std::process::exit(exitcode::NOINPUT);
    }

    // Build the 1x and 2x sprite maps, applying the regular/retina fallback in both directions
    // (build-sprite.js:36-41). The pixel ratio of each sprite follows its actual source file
    // name, so a retina image that falls back into the 1x spritesheet keeps `pixelRatio: 2`.
    let mut regular_sprites = BTreeMap::new();
    let mut retina_sprites = BTreeMap::new();
    for (name, (regular_path, retina_path)) in pairs {
        let regular = regular_path.clone().or_else(|| retina_path.clone());
        let retina = retina_path.or_else(|| regular_path.clone());
        if let Some(path) = regular {
            match load_png_sprite(&path) {
                Ok(sprite) => {
                    regular_sprites.insert(name.clone(), sprite);
                }
                Err(e) => {
                    eprintln!("Error: cannot load PNG sprite from {path:?} ({e})");
                    std::process::exit(exitcode::DATAERR);
                }
            }
        }
        if let Some(path) = retina {
            match load_png_sprite(&path) {
                Ok(sprite) => {
                    retina_sprites.insert(name, sprite);
                }
                Err(e) => {
                    eprintln!("Error: cannot load PNG sprite from {path:?} ({e})");
                    std::process::exit(exitcode::DATAERR);
                }
            }
        }
    }

    // Generate and save the 1x spritesheet (<OUTPUT>.png / <OUTPUT>.json).
    let sheet1 = build_spritesheet(regular_sprites, args.unique, false, args.spacing);
    save_spritesheet_png(&sheet1, &args.output);
    if let Err(e) = save_png_pair_index(&sheet1, &args.output, args.minify_index_file) {
        eprintln!("Error: could not save sprite index to {} ({e})", args.output);
        std::process::exit(exitcode::IOERR);
    }

    // Generate and save the 2x spritesheet (<OUTPUT>@2x.png / <OUTPUT>@2x.json).
    let output2 = format!("{}@2x", args.output);
    let sheet2 = build_spritesheet(retina_sprites, args.unique, false, args.spacing);
    save_spritesheet_png(&sheet2, &output2);
    if let Err(e) = save_png_pair_index(&sheet2, &output2, args.minify_index_file) {
        eprintln!("Error: could not save sprite index to {output2} ({e})");
        std::process::exit(exitcode::IOERR);
    }
}

/// Save the spritesheet PNG to `{file_prefix}.png`.
fn save_spritesheet_png(spritesheet: &Spritesheet, file_prefix: &str) {
    let spritesheet_path = format!("{file_prefix}.png");
    if let Err(e) = spritesheet.save_spritesheet(&spritesheet_path) {
        eprintln!("Error: could not save spritesheet to {spritesheet_path} ({e})");
        std::process::exit(exitcode::IOERR);
    };
}

/// Save the sprite index with an explicit `sdf` field on every sprite (`"sdf": false` is written
/// rather than omitted), mirroring build-sprite.js, whose index always carries the field.
fn save_png_pair_index(
    spritesheet: &Spritesheet,
    file_name_prefix: &str,
    minify: bool,
) -> std::io::Result<()> {
    let mut explicit = BTreeMap::new();
    for (name, description) in spritesheet.get_index() {
        let value = json!({
            "height": description.height,
            "width": description.width,
            "x": description.x,
            "y": description.y,
            "pixelRatio": description.pixel_ratio,
            "sdf": description.sdf,
        });
        explicit.insert(name.clone(), value);
    }
    let mut file = std::fs::File::create(format!("{file_name_prefix}.json"))?;
    let json_string = if minify {
        serde_json::to_string(&explicit)?
    } else {
        serde_json::to_string_pretty(&explicit)?
    };
    use std::io::Write as _;
    write!(file, "{json_string}")?;
    Ok(())
}

/// Load a PNG file as a sprite. The sprite's pixel ratio is 2 when the source file name contains
/// `@2x`, and 1 otherwise; its `sdf` flag follows the source file path (see
/// [`Sprite::from_png`]).
fn load_png_sprite(path: &PathBuf) -> spreet::SpreetResult<Sprite> {
    let path_str = path.to_string_lossy();
    let pixel_ratio = if path_str.contains("@2x") { 2 } else { 1 };
    Sprite::from_png(path, pixel_ratio)
}
