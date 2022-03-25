use std::cmp::max;
use std::collections::BTreeMap;
use std::fs;

use clap::Parser;
use resvg::render;
use tiny_skia::{Pixmap, PixmapPaint, Transform};
use usvg::{FitTo, Options, Tree};
use walkdir::WalkDir;

use spreet::fs::{is_interesting_input, save_sprite_index_file};
use spreet::sprite::SpriteDescription;

mod cli;

fn main() {
    let args = cli::Cli::parse();

    // The ratio between the pixels in an SVG image and the pixels in the resulting PNG sprite. A
    // value of 2 means the PNGs will be double the size of the SVG images.
    let pixel_ratio = if args.retina { 2 } else { args.ratio };

    // Collect the file paths for all SVG images in the input directory.
    let mut file_paths = Vec::new();
    let walker = WalkDir::new(&args.input).follow_links(true).into_iter();
    for entry in walker.filter_entry(is_interesting_input).flatten() {
        file_paths.push(entry.into_path());
    }
    if file_paths.is_empty() {
        eprintln!("Error: no SVGs found in {:?}", &args.input);
        std::process::exit(exitcode::NOINPUT);
    }

    // Read all SVG data from the input files and convert them to bitmap sprites.
    let mut sprites = BTreeMap::new();
    let mut spritesheet_rects = rectangle_pack::GroupedRectsToPlace::new();
    let mut total_pixels = 0; // Pixels in the sprites. Used to decide the size of the spritesheet.
    let mut max_sprite_width = 0;
    let mut max_sprite_height = 0;
    for file_path in file_paths {
        if let Ok(svg_data) = fs::read(&file_path) {
            let sprite_name = format!("{}", file_path.file_stem().unwrap().to_string_lossy());
            let fit_to = FitTo::Zoom(pixel_ratio as f32);
            let tree = Tree::from_data(&svg_data, &Options::default().to_ref());
            if let Ok(t) = tree {
                // A valid SVG document has been parsed from the file contents, now convert
                // it to a bitmap.
                let size = fit_to.fit_to(t.svg_node().size.to_screen_size()).unwrap();
                let mut sprite = Pixmap::new(size.width(), size.height()).unwrap();
                render(&t, fit_to, Transform::default(), sprite.as_mut());
                // Set aside a rectangular space for the bitmap sprite. This will be packed
                // into the spritesheet later.
                spritesheet_rects.push_rect(
                    sprite_name.clone(),
                    Some(vec![1]),
                    rectangle_pack::RectToInsert::new(sprite.width(), sprite.height(), 1),
                );
                total_pixels += sprite.height() * sprite.width();
                max_sprite_width = max(sprite.width(), max_sprite_width);
                max_sprite_height = max(sprite.height(), max_sprite_height);
                sprites.insert(sprite_name, sprite);
            }
        }
    }

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
            SpriteDescription {
                height: location.height(),
                width: location.width(),
                pixel_ratio,
                x: location.x(),
                y: location.y(),
            },
        );
    }

    // Save the spritesheet (what Mapbox call the image file) as a PNG image.
    // https://docs.mapbox.com/mapbox-gl-js/style-spec/sprite/#image-file
    match spritesheet.save_png(format!("{}.png", args.output)) {
        Ok(()) => {}
        Err(e) => {
            eprintln!(
                "Error: could not save spritesheet to {} ({})",
                args.output, e
            );
            std::process::exit(exitcode::IOERR);
        }
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