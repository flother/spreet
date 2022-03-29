use std::collections::BTreeMap;
use std::path::Path;

use rectangle_pack::GroupedRectsToPlace;
use resvg::render;
use serde::Serialize;
use tiny_skia::{Pixmap, Transform};
use usvg::{FitTo, Tree};

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

pub fn sprite_name(path: &Path) -> String {
    format!("{}", path.file_stem().unwrap().to_string_lossy())
}

// Generate a bitmap image from an SVG image.
//
// The bitmap is generated at the given pixel ratio. A ratio of 2 means the bitmap image will be
// scaled to be double the size of the SVG image.
pub fn generate_pixmap_from_svg(svg: Tree, pixel_ratio: u8) -> Option<Pixmap> {
    let fit_to = FitTo::Zoom(pixel_ratio as f32);
    let size = fit_to.fit_to(svg.svg_node().size.to_screen_size())?;
    let mut sprite = Pixmap::new(size.width(), size.height())?;
    render(&svg, fit_to, Transform::default(), sprite.as_mut());
    Some(sprite)
}

// Set aside a rectangular space for each bitmapped sprite.
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
