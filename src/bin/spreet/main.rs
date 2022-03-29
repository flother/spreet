use std::cmp::max;
use std::collections::BTreeMap;

use clap::Parser;
use tiny_skia::{Pixmap, PixmapPaint, Transform};

use spreet::fs::{get_svg_input_paths, load_svg, save_sprite_index_file, save_spritesheet};
use spreet::sprite;

mod cli;

fn main() {
    let args = cli::Cli::parse();

    // The ratio between the pixels in an SVG image and the pixels in the resulting PNG sprite. A
    // value of 2 means the PNGs will be double the size of the SVG images.
    let pixel_ratio = if args.retina { 2 } else { args.ratio };

    // Collect the file paths for all SVG images in the input directory.
    let svg_paths = get_svg_input_paths(&args.input);
    if svg_paths.is_empty() {
        eprintln!("Error: no SVGs found in {:?}", &args.input);
        std::process::exit(exitcode::NOINPUT);
    }

    // Read from all the input SVG files, convert them into bitmaps at the correct pixel ratio, and
    // store them in a map. The keys are the SVG filenames without the `.svg` extension. The
    // bitmapped SVGs will be added to the spritesheet, and the keys will be used as the unique
    // sprite ids in the JSON index file.
    let mut sprites = BTreeMap::new();
    for svg_path in svg_paths {
        match load_svg(&svg_path) {
            Ok(svg) => {
                sprites.insert(
                    sprite::sprite_name(&svg_path),
                    sprite::generate_pixmap_from_svg(svg, pixel_ratio).unwrap(),
                );
            }
            Err(_) => {
                eprintln!("{:?}: not a valid SVG image", &svg_path);
                std::process::exit(exitcode::DATAERR);
            }
        }
    }
    if sprites.is_empty() {
        eprintln!("Error: no valid SVGs found in {:?}", &args.input);
        std::process::exit(exitcode::NOINPUT);
    }

    // Set aside a rectangular space for the bitmap sprite. This will be packed into the spritesheet
    // later.
    let spritesheet_rects = sprite::generate_spritesheet_rects(&sprites);
    // Get the width the widest sprite. Used to enforce a minimum width for the spritesheet.
    let max_sprite_width = sprites
        .values()
        .max_by(|x, y| x.width().cmp(&y.width()))
        .unwrap()
        .width();
    // Get the height the tallest sprite. Used to enforce a minimum height for the spritesheet.
    let max_sprite_height = sprites
        .values()
        .max_by(|x, y| x.height().cmp(&y.height()))
        .unwrap()
        .height();
    // Calculate total number of pixels in the sprites. Used to decide the size of the spritesheet.
    let total_pixels = sprites
        .values()
        .fold(0u32, |sum, p| sum + (p.width() * p.height()));

    // The rectangle-pack library doesn't support automatically resizing the target bin if it runs
    // out of space. But if you give it too large a space --- say, 4096×4096 pixels --- then it will
    // do its best to use all that space. We want the most compact form possible, so the solution is
    // to start with a square exactly the size of the sprites' total pixels and expand in 0.1
    // increments each time it runs out of space.
    let rectangle_placements;
    let mut bin_dimensions;
    let mut i = 1.0;
    loop {
        // Set up a single target bin. (We only need one because we only want one spritesheet.)
        // It's usually a square but it's always at least the width of the widest sprite, and the
        // height of the tallest sprite, so it may be rectangular. Attempt to pack all the sprites
        // into the bin.
        bin_dimensions = (total_pixels as f32 * i).sqrt().ceil() as u32;
        let mut target_bins = BTreeMap::new();
        target_bins.insert(
            "target_bin",
            rectangle_pack::TargetBin::new(
                max(bin_dimensions, max_sprite_width),
                max(bin_dimensions, max_sprite_height),
                1,
            ),
        );
        let result = rectangle_pack::pack_rects(
            &spritesheet_rects,
            &mut target_bins,
            &rectangle_pack::volume_heuristic,
            &rectangle_pack::contains_smallest_box,
        );
        if let Ok(placements) = result {
            rectangle_placements = placements;
            break;
        } else if i >= 50.0 {
            // This is to stop an infinite loop. If we've reached the point where the bin-packing
            // algorithm can't fit the sprites into a square fifty times the size of the sprites
            // combined, we're in trouble. (This would likely come about in a situation where there
            // is an extraordinary long and tall sprite.)
            eprintln!("Error: could not pack the sprites within an area fifty times their size.");
            std::process::exit(exitcode::DATAERR);
        }
        i += 0.1;
    }
    // There might be some unused space in the target bin --- not all the pixels on the right/bottom
    // edges may have been used. Count the pixels in use so we can strip off any empty edges in the
    // final spritesheet. The won't strip any transparent pixels within a sprite, just unused pixels
    // around the sprites.
    let mut bin_height = 0;
    let mut bin_width = 0;
    for (_, location) in rectangle_placements.packed_locations().values() {
        let location_height = location.y() + location.height();
        if location_height > bin_height {
            bin_height = location_height;
        }
        let location_width = location.x() + location.width();
        if location_width > bin_width {
            bin_width = location_width;
        }
    }

    // This is the real meat of the program. Here we pack the sprite bitmaps into the spritesheet,
    // using the locations from the previous step, and store those locations in the vector that will
    // be output as the sprite index file.
    let mut sprite_index = BTreeMap::new();
    let mut spritesheet = Pixmap::new(bin_width, bin_height).unwrap();
    let pixmap_paint = PixmapPaint::default();
    let pixmap_transform = Transform::default();
    for (sprite_name, rectangle) in rectangle_placements.packed_locations().iter() {
        let sprite = sprites.get(sprite_name).unwrap();
        let location = rectangle.1;
        spritesheet.draw_pixmap(
            location.x() as i32,
            location.y() as i32,
            sprite.as_ref(),
            &pixmap_paint,
            pixmap_transform,
            None,
        );
        sprite_index.insert(
            sprite_name,
            sprite::SpriteDescription {
                height: location.height(),
                width: location.width(),
                pixel_ratio,
                x: location.x(),
                y: location.y(),
            },
        );
    }

    let spritesheet_path = format!("{}.png", args.output);
    if let Err(e) = save_spritesheet(&spritesheet_path, spritesheet) {
        eprintln!(
            "Error: could not save spritesheet to {} ({})",
            spritesheet_path, e
        );
        std::process::exit(exitcode::IOERR);
    };
    // Save the index file (a JSON document containing a description of each image contained in the
    // sprite) to a local file with the same name as the spritesheet.
    // https://docs.mapbox.com/mapbox-gl-js/style-spec/sprite/#index-file
    match save_sprite_index_file(&args.output, sprite_index) {
        Ok(_) => {}
        Err(e) => {
            eprintln!(
                "Error: could not save sprite index to {} ({})",
                args.output, e
            );
            std::process::exit(exitcode::IOERR);
        }
    };
}
