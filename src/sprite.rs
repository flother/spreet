use std::cmp::max;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use oxipng::optimize_from_memory;
use rectangle_pack::GroupedRectsToPlace;
use resvg::render;
use serde::Serialize;
use tiny_skia::{Pixmap, PixmapPaint, Transform};
use usvg::{FitTo, Tree};

use crate::error::Error;

/// A description of a sprite image within a spritesheet. Used for the JSON output required by a
/// Mapbox Style Specification [index file].
///
/// [index file]: https://docs.mapbox.com/mapbox-gl-js/style-spec/sprite/#index-file
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpriteDescription {
    pub height: u32,
    pub pixel_ratio: u8,
    pub width: u32,
    pub x: u32,
    pub y: u32,
}

/// A bitmapped spritesheet and its matching index.
pub struct Spritesheet {
    sheet: Pixmap,
    index: BTreeMap<String, SpriteDescription>,
}

impl Spritesheet {
    pub fn new(sprites: BTreeMap<String, Pixmap>, pixel_ratio: u8, max_size: f32) -> Option<Self> {
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
        let spritesheet_rects = generate_spritesheet_rects(&sprites);
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
            } else if i >= max_size {
                // This is to stop an infinite loop. If we've reached the point where the bin-packing
                // algorithm can't fit the sprites into a square `max_size` times the size of the
                // sprites combined, we're in trouble. (This would likely come about in a situation
                // where there is an extraordinary long and tall sprite.)
                return None;
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

        // This is the real meat of Spreet. Here we pack the sprite bitmaps into the spritesheet,
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
                sprite_name.clone(),
                SpriteDescription {
                    height: location.height(),
                    width: location.width(),
                    pixel_ratio,
                    x: location.x(),
                    y: location.y(),
                },
            );
        }

        Some(Spritesheet {
            sheet: spritesheet,
            index: sprite_index,
        })
    }

    /// Saves the spritesheet to a local file named `path`.
    ///
    /// A spritesheet, called an [image file] in the Mapbox Style Specification, is a PNG image
    /// containing all the individual sprite images. The `spritesheet` `Pixmap` is converted to an
    /// in-memory PNG, optimised using the [`oxipng`] library, and saved to a local file.
    ///
    /// The spritesheet will match an index file that can be saved with [`save_sprite_index_file`].
    ///
    /// [image file]: https://docs.mapbox.com/mapbox-gl-js/style-spec/sprite/#image-file
    /// [`oxipng`]: https://github.com/shssoichiro/oxipng
    pub fn save_spritesheet(&self, path: &str) -> Result<(), Error> {
        let spritesheet_png = self.sheet.encode_png()?;
        let spritesheet_png =
            optimize_from_memory(spritesheet_png.as_slice(), &oxipng::Options::default())?;
        std::fs::write(path, spritesheet_png)?;
        Ok(())
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
    pub fn save_index(&self, file_name_prefix: &str) -> std::io::Result<()> {
        let mut file = File::create(format!("{}.json", file_name_prefix))?;
        let json_string = serde_json::to_string_pretty(&self.index)?;
        write!(file, "{}", json_string)?;
        Ok(())
    }
}

/// Returns the name (unique id within a spritesheet) taken from a file.
pub fn sprite_name(path: &Path) -> String {
    format!("{}", path.file_stem().unwrap().to_string_lossy())
}

/// Generate a bitmap image from an SVG image.
///
/// The bitmap is generated at the given pixel ratio. A ratio of 2 means the bitmap image will be
/// scaled to be double the size of the SVG image.
pub fn generate_pixmap_from_svg(svg: Tree, pixel_ratio: u8) -> Option<Pixmap> {
    let fit_to = FitTo::Zoom(pixel_ratio as f32);
    let size = fit_to.fit_to(svg.svg_node().size.to_screen_size())?;
    let mut sprite = Pixmap::new(size.width(), size.height())?;
    render(&svg, fit_to, Transform::default(), sprite.as_mut());
    Some(sprite)
}

/// Set aside a rectangular space for each bitmapped sprite.
pub fn generate_spritesheet_rects(
    sprites: &BTreeMap<String, Pixmap>,
) -> GroupedRectsToPlace<String, i32> {
    let mut spritesheet_rects = rectangle_pack::GroupedRectsToPlace::new();
    for (sprite_name, sprite) in sprites {
        spritesheet_rects.push_rect(
            sprite_name.clone(),
            Some(vec![1]),
            rectangle_pack::RectToInsert::new(sprite.width(), sprite.height(), 1),
        );
    }
    spritesheet_rects
}
