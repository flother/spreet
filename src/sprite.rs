use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use crunch::{Item, PackedItem, PackedItems, Rotation};
use multimap::MultiMap;
use oxipng::optimize_from_memory;
use resvg::tiny_skia::{Pixmap, PixmapPaint, Transform};
use resvg::usvg::Tree;
use serde::Serialize;

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

/// Builder pattern for `Spritesheet`: construct a `Spritesheet` object using calls to a builder
/// helper.
#[derive(Default)]
pub struct SpritesheetBuilder {
    sprites: Option<BTreeMap<String, Pixmap>>,
    references: Option<MultiMap<String, String>>,
    pixel_ratio: u8,
}

impl SpritesheetBuilder {
    pub fn new() -> Self {
        Self {
            sprites: None,
            references: None,
            pixel_ratio: 1,
        }
    }

    pub fn sprites(&mut self, sprites: BTreeMap<String, Pixmap>) -> &mut Self {
        self.sprites = Some(sprites);
        self
    }

    pub fn pixel_ratio(&mut self, pixel_ratio: u8) -> &mut Self {
        self.pixel_ratio = pixel_ratio;
        self
    }

    // Remove any duplicate sprites from the spritesheet's sprites. This is used to let spritesheets
    // include only unique sprites, with multiple references to the same sprite in the index file.
    pub fn make_unique(&mut self) -> &mut Self {
        match &self.sprites {
            Some(sprites) => {
                let mut unique_sprites = BTreeMap::new();
                let mut references = MultiMap::new();
                let mut names_for_sprites: BTreeMap<Vec<u8>, String> = BTreeMap::new();
                for (name, sprite) in sprites {
                    let sprite_data = sprite.encode_png().unwrap();
                    if let Some(existing_sprite_name) = names_for_sprites.get(&sprite_data) {
                        references.insert(existing_sprite_name.clone(), name.clone());
                    } else {
                        names_for_sprites.insert(sprite_data, name.clone());
                        unique_sprites.insert(name.clone(), sprite.clone());
                    }
                }
                self.sprites = Some(unique_sprites);
                self.references = Some(references);
            }
            None => {
                self.references = None;
            }
        }
        self
    }

    pub fn generate(&self) -> Option<Spritesheet> {
        Spritesheet::new(
            self.sprites.clone().unwrap_or_default(),
            self.references.clone().unwrap_or_default(),
            self.pixel_ratio,
        )
    }
}

// A bitmapped spritesheet and its matching index.
pub struct Spritesheet {
    sheet: Pixmap,
    index: BTreeMap<String, SpriteDescription>,
}

impl Spritesheet {
    pub fn new(
        sprites: BTreeMap<String, Pixmap>,
        references: MultiMap<String, String>,
        pixel_ratio: u8,
    ) -> Option<Self> {
        // The items are the rectangles that we want to pack into the smallest space possible. We
        // don't need to pass the pixels themselves, just the unique name for each sprite.
        let items: Vec<Item<String>> = sprites
            .iter()
            .map(|(name, image)| {
                Item::new(
                    name.clone(),
                    image.width() as usize,
                    image.height() as usize,
                    Rotation::None,
                )
            })
            .collect();
        // Minimum area required for the spreadsheet (i.e. 100% coverage).
        let min_area: usize = sprites
            .values()
            .map(|i| i.width() as usize * i.height() as usize)
            .sum();
        match crunch::pack_into_po2(min_area * 10, items) {
            Ok(PackedItems { items: packed, .. }) => {
                // There might be some unused space in the packed items --- not all the pixels on
                // the right/bottom edges may have been used. Count the pixels in use so we can
                // strip off any empty edges in the final spritesheet. The won't strip any
                // transparent pixels within a sprite, just unused pixels around the sprites.
                let bin_width = packed
                    .iter()
                    .map(|PackedItem { rect, .. }| rect.right())
                    .max()? as u32;
                let bin_height = packed
                    .iter()
                    .map(|PackedItem { rect, .. }| rect.bottom())
                    .max()? as u32;
                // This is the meat of Spreet. Here we pack the sprite bitmaps into the spritesheet,
                // using the rectangle locations from the previous step, and store those locations
                // in the vector that will be output as the sprite index file.
                let mut sprite_index = BTreeMap::new();
                let mut spritesheet = Pixmap::new(bin_width, bin_height)?;
                let pixmap_paint = PixmapPaint::default();
                let pixmap_transform = Transform::default();
                for PackedItem {
                    rect,
                    data: sprite_name,
                } in &packed
                {
                    let sprite = sprites.get(sprite_name)?;
                    spritesheet.draw_pixmap(
                        rect.x as i32,
                        rect.y as i32,
                        sprite.as_ref(),
                        &pixmap_paint,
                        pixmap_transform,
                        None,
                    );
                    sprite_index.insert(
                        sprite_name.clone(),
                        SpriteDescription {
                            height: rect.h as u32,
                            width: rect.w as u32,
                            pixel_ratio,
                            x: rect.x as u32,
                            y: rect.y as u32,
                        },
                    );
                    // If multiple names are used for a unique sprite, insert an entry in the index
                    // for each of the other names. This is to allow for multiple names to reference
                    // the same SVG image without having to include it in the spritesheet multiple
                    // times. The `--unique` // command-flag can be used to control this behaviour.
                    if let Some(other_sprite_names) = references.get_vec(sprite_name) {
                        for other_sprite_name in other_sprite_names {
                            sprite_index.insert(
                                other_sprite_name.clone(),
                                SpriteDescription {
                                    height: rect.h as u32,
                                    width: rect.w as u32,
                                    pixel_ratio,
                                    x: rect.x as u32,
                                    y: rect.y as u32,
                                },
                            );
                        }
                    }
                }
                Some(Spritesheet {
                    sheet: spritesheet,
                    index: sprite_index,
                })
            }
            Err(_) => None,
        }
    }

    pub fn build() -> SpritesheetBuilder {
        SpritesheetBuilder::new()
    }

    /// Saves the spritesheet to a local file named `path`.
    ///
    /// A spritesheet, called an [image file] in the Mapbox Style Specification, is a PNG image
    /// containing all the individual sprite images. The `spritesheet` `Pixmap` is converted to an
    /// in-memory PNG, optimised using the [`oxipng`] library, and saved to a local file.
    ///
    /// The spritesheet will match an index file that can be saved with [`Self::save_index`].
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
    /// The index file will match a spritesheet that can be saved with [`Self::save_spritesheet`].
    ///
    /// [index file]: https://docs.mapbox.com/mapbox-gl-js/style-spec/sprite/#index-file
    pub fn save_index(&self, file_name_prefix: &str, minify: bool) -> std::io::Result<()> {
        let mut file = File::create(format!("{file_name_prefix}.json"))?;
        let json_string = if minify {
            serde_json::to_string(&self.index)?
        } else {
            serde_json::to_string_pretty(&self.index)?
        };
        write!(file, "{json_string}")?;
        Ok(())
    }
}

/// Returns the name (unique id within a spritesheet) taken from a file.
///
/// The unique sprite name is the relative path from `path` to `base_path`
/// without the file extension.
pub fn sprite_name(path: &Path, base_path: &Path) -> String {
    let abs_path = path.canonicalize().unwrap();
    let abs_base_path = base_path.canonicalize().unwrap();
    let rel_path = abs_path.strip_prefix(abs_base_path).unwrap();
    let file_stem = path.file_stem().unwrap();

    if let Some(parent) = rel_path.parent() {
        format!("{}", parent.join(file_stem).to_string_lossy())
    } else {
        format!("{}", file_stem.to_string_lossy())
    }
}

/// Generate a bitmap image from an SVG image.
///
/// The bitmap is generated at the given pixel ratio. A ratio of 2 means the bitmap image will be
/// scaled to be double the size of the SVG image.
pub fn generate_pixmap_from_svg(svg: &Tree, pixel_ratio: u8) -> Option<Pixmap> {
    let rtree = resvg::Tree::from_usvg(svg);
    let pixmap_size = rtree
        .size
        .to_int_size()
        .scale_by(pixel_ratio as f32)
        .unwrap();
    let mut pixmap = Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
    let render_ts = Transform::from_scale(pixel_ratio as f32, pixel_ratio as f32);
    rtree.render(render_ts, &mut pixmap.as_mut());
    Some(pixmap)
}
