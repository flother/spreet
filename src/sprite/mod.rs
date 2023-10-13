use std::collections::btree_map::Entry;
use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::Write;
use std::path::Path;

use crunch::{Item, PackedItem, PackedItems, Rotation};
use multimap::MultiMap;
use oxipng::optimize_from_memory;
use resvg::tiny_skia::{Pixmap, PixmapPaint, Transform};
use resvg::usvg::{NodeExt, Rect, Tree};
use serde::Serialize;

use self::serialize::{serialize_rect, serialize_stretch_x_area, serialize_stretch_y_area};
pub use crate::error::{SpreetError, SpreetResult};

mod serialize;

/// A single icon within a spritesheet.
///
/// A sprite is a rectangular icon stored as an SVG image and converted to a bitmap. The bitmap is
/// saved to a spritesheet.
#[derive(Clone)]
pub struct Sprite {
    /// Parsed source SVG image.
    pub tree: Tree,
    /// Ratio determining the size the destination pixels compared to the source pixels. A ratio of
    /// 2 means the bitmap will be scaled to be twice the size of the SVG image.
    pub pixel_ratio: u8,
}

impl Sprite {
    /// Generate a bitmap image from the sprite's SVG tree.
    ///
    /// The bitmap is generated at the sprite's [pixel ratio](Self::pixel_ratio).
    pub fn pixmap(&self) -> Option<Pixmap> {
        let rtree = resvg::Tree::from_usvg(&self.tree);
        let pixmap_size = rtree.size.to_int_size().scale_by(self.pixel_ratio as f32)?;
        let mut pixmap = Pixmap::new(pixmap_size.width(), pixmap_size.height())?;
        let render_ts = Transform::from_scale(self.pixel_ratio.into(), self.pixel_ratio.into());
        rtree.render(render_ts, &mut pixmap.as_mut());
        Some(pixmap)
    }

    /// Metadata for a [stretchable icon].
    ///
    /// Describes the content area of an icon as a [`Rect`]. The metadata comes from the bounding
    /// box of an element in the SVG image that has the id `mapbox-content`.
    ///
    /// Most icons do not specify a content area. But if it is present and the MapLibre/Mapbox map
    /// symbol uses [`icon-text-fit`], the symbol's text will be fitted inside this content box.
    ///
    /// [stretchable icon]: https://github.com/mapbox/mapbox-gl-js/issues/8917
    /// [`icon-text-fit`]: https://maplibre.org/maplibre-style-spec/layers/#layout-symbol-icon-text-fit
    pub fn content_area(&self) -> Option<Rect> {
        self.get_node_bbox("mapbox-content")
    }

    /// Metadata for a [stretchable icon].
    ///
    /// Describes the horizontal position of areas that can be stretched. There may be multiple
    /// areas. The metadata comes from the bounding boxes of elements in the SVG image that have
    /// ids like `mapbox-stretch-x-1`. Although the entire bounding box is provided, only the left
    /// and right edges are stored in the index file and used by MapLibre/Mapbox to define the
    /// stretchable area.
    ///
    /// Most icons do not specify stretchable areas. See also [`Sprite::content_area`].
    ///
    /// [stretchable icon]: https://github.com/mapbox/mapbox-gl-js/issues/8917
    pub fn stretch_x_areas(&self) -> Option<Vec<Rect>> {
        let mut values = vec![];
        // First look for an SVG element with the id `mapbox-stretch-x`.
        if let Some(rect) = self.get_node_bbox("mapbox-stretch-x") {
            values.push(rect);
        }
        // Next look for SVG elements with ids like `mapbox-stretch-x-1`. As soon as one is missing,
        // stop looking.
        for i in 1.. {
            if let Some(rect) = self.get_node_bbox(format!("mapbox-stretch-x-{i}").as_str()) {
                values.push(rect);
            } else {
                break;
            }
        }
        if values.is_empty() {
            // If there are no SVG elements with `mapbox-stretch-x` ids, check for an element with
            // the id `mapbox-stretch`. That's a shorthand for stretch-x and stretch-y. If that
            // exists, use its horizontal coordinates.
            self.get_node_bbox("mapbox-stretch").map(|rect| vec![rect])
        } else {
            Some(values)
        }
    }

    /// Metadata for a [stretchable icon].
    ///
    /// Describes the vertical position of areas that can be stretched. There may be multiple areas.
    /// The metadata comes from the bounding boxes of elements in the SVG image that have ids like
    /// `mapbox-stretch-y-1`. Although the entire bounding box is provided, only the top and bottom
    /// edges are stored in the index file and used by MapLibre/Mapbox to define the stretchable
    /// area.
    ///
    /// Most icons do not specify stretchable areas. See also [`Sprite::content_area`].
    ///
    /// [stretchable icon]: https://github.com/mapbox/mapbox-gl-js/issues/8917
    pub fn stretch_y_areas(&self) -> Option<Vec<Rect>> {
        let mut values = vec![];
        // First look for an SVG element with the id `mapbox-stretch-y`.
        if let Some(rect) = self.get_node_bbox("mapbox-stretch-y") {
            values.push(rect);
        }
        // Next look for SVG elements with ids like `mapbox-stretch-y-1`. As soon as one is missing,
        // stop looking.
        for i in 1.. {
            if let Some(rect) = self.get_node_bbox(format!("mapbox-stretch-y-{i}").as_str()) {
                values.push(rect);
            } else {
                break;
            }
        }
        if values.is_empty() {
            // If there are no SVG elements with `mapbox-stretch-x` ids, check for an element with
            // the id `mapbox-stretch`. That's a shorthand for stretch-x and stretch-y. If that
            // exists, use its vertical coordinates.
            self.get_node_bbox("mapbox-stretch").map(|rect| vec![rect])
        } else {
            Some(values)
        }
    }

    /// Find a node in the SVG tree with a given id, and return its bounding box with coordinates
    /// multiplied by the sprite's pixel ratio.
    fn get_node_bbox(&self, id: &str) -> Option<Rect> {
        self.tree.node_by_id(id)?.calculate_bbox().map(|bbox| {
            let ratio = self.pixel_ratio as f32;
            Rect::from_ltrb(
                bbox.left() * ratio,
                bbox.top() * ratio,
                bbox.right() * ratio,
                bbox.bottom() * ratio,
            )
            .unwrap()
        })
    }
}

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
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_rect"
    )]
    pub content: Option<Rect>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_stretch_x_area"
    )]
    pub stretch_x: Option<Vec<Rect>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_stretch_y_area"
    )]
    pub stretch_y: Option<Vec<Rect>>,
}

/// Builder pattern for `Spritesheet`: construct a `Spritesheet` object using calls to a builder
/// helper.
#[derive(Default, Clone)]
pub struct SpritesheetBuilder {
    sprites: Option<BTreeMap<String, Sprite>>,
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

    pub fn sprites(&mut self, sprites: BTreeMap<String, Sprite>) -> &mut Self {
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
        match self.sprites.take() {
            Some(sprites) => {
                let mut unique_sprites = BTreeMap::new();
                let mut references = MultiMap::new();
                let mut names_for_sprites: BTreeMap<Vec<u8>, String> = BTreeMap::new();
                for (name, sprite) in sprites {
                    let sprite_data = sprite.pixmap().unwrap().encode_png().unwrap();
                    match names_for_sprites.entry(sprite_data) {
                        Entry::Occupied(existing_sprite_name) => {
                            references.insert(existing_sprite_name.get().clone(), name);
                        }
                        Entry::Vacant(entry) => {
                            entry.insert(name.clone());
                            unique_sprites.insert(name, sprite);
                        }
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

    pub fn generate(self) -> Option<Spritesheet> {
        Spritesheet::new(
            self.sprites.unwrap_or_default(),
            self.references.unwrap_or_default(),
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
        sprites: BTreeMap<String, Sprite>,
        references: MultiMap<String, String>,
        pixel_ratio: u8,
    ) -> Option<Self> {
        let pixmaps: HashMap<&String, Pixmap> = sprites
            .iter()
            .map(|(name, sprite)| (name, sprite.pixmap().unwrap()))
            .collect();
        // The items are the rectangles that we want to pack into the smallest space possible. We
        // don't need to pass the pixels themselves, just the unique name for each sprite.
        let items: Vec<Item<String>> = sprites
            .keys()
            .map(|name| {
                let image = pixmaps.get(name).unwrap();
                Item::new(
                    name.clone(),
                    image.width() as usize,
                    image.height() as usize,
                    Rotation::None,
                )
            })
            .collect();
        // Minimum area required for the spritesheet (i.e. 100% coverage).
        let min_area: usize = sprites
            .keys()
            .map(|name| {
                let image = pixmaps.get(name).unwrap();
                image.width() as usize * image.height() as usize
            })
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
                        pixmaps.get(sprite_name).unwrap().as_ref(),
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
                            content: sprite.content_area(),
                            stretch_x: sprite.stretch_x_areas(),
                            stretch_y: sprite.stretch_y_areas(),
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
                                    content: sprite.content_area(),
                                    stretch_x: sprite.stretch_x_areas(),
                                    stretch_y: sprite.stretch_y_areas(),
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
            Err(()) => None,
        }
    }

    pub fn build() -> SpritesheetBuilder {
        SpritesheetBuilder::new()
    }

    /// Encode the spritesheet to the in-memory PNG image.
    ///
    /// The `spritesheet` `Pixmap` is converted to an in-memory PNG, optimised using the [`oxipng`]
    /// library.
    ///
    /// The spritesheet will match an index that can be retrieved with [`Self::get_index`].
    ///
    /// [`oxipng`]: https://github.com/shssoichiro/oxipng
    pub fn encode_png(&self) -> SpreetResult<Vec<u8>> {
        Ok(optimize_from_memory(
            self.sheet.encode_png()?.as_slice(),
            &oxipng::Options::default(),
        )?)
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
    pub fn save_spritesheet<P: AsRef<Path>>(&self, path: P) -> SpreetResult<()> {
        Ok(std::fs::write(path, self.encode_png()?)?)
    }

    /// Get the `sprite_index` that can be serialized to JSON.
    ///
    /// An [index file] is defined in the Mapbox Style Specification as a JSON document containing a
    /// description of each sprite within a spritesheet. It contains the width, height, x and y
    /// positions, and pixel ratio of the sprite.
    ///
    /// The index file will match a spritesheet that can be saved with [`Self::save_spritesheet`].
    ///
    /// [index file]: https://docs.mapbox.com/mapbox-gl-js/style-spec/sprite/#index-file
    pub fn get_index(&self) -> &BTreeMap<String, SpriteDescription> {
        &self.index
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
            serde_json::to_string(&self.get_index())?
        } else {
            serde_json::to_string_pretty(&self.get_index())?
        };
        write!(file, "{json_string}")?;
        Ok(())
    }
}

/// Returns the name (unique id within a spritesheet) taken from a file.
///
/// The unique sprite name is the relative path from `path` to `base_path`
/// without the file extension.
///
/// # Errors
///
/// This function will return an error if:
///
/// - `path` does not exist
/// - `abs_path` does not exist
/// - `abs_path` is not a ancestor of `path`
pub fn sprite_name<P1: AsRef<Path>, P2: AsRef<Path>>(
    path: P1,
    base_path: P2,
) -> SpreetResult<String> {
    let abs_path = path.as_ref().canonicalize()?;
    let abs_base_path = base_path.as_ref().canonicalize()?;
    let Ok(rel_path) = abs_path.strip_prefix(abs_base_path) else {
        return Err(SpreetError::PathError(path.as_ref().to_path_buf()));
    };

    let Some(file_stem) = path.as_ref().file_stem() else {
        return Err(SpreetError::PathError(path.as_ref().to_path_buf()));
    };
    if let Some(parent) = rel_path.parent() {
        Ok(format!("{}", parent.join(file_stem).to_string_lossy()))
    } else {
        Ok(format!("{}", file_stem.to_string_lossy()))
    }
}
