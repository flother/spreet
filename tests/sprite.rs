use std::collections::BTreeMap;
use std::path::Path;

use assert_matches::assert_matches;
use resvg::usvg::{Options, Rect, Tree};
use spreet::{load_svg, sprite_name, SpreetError, Sprite, Spritesheet};

#[test]
fn sprite_name_works_with_root_files() {
    assert_eq!(
        sprite_name(
            Path::new("./tests/fixtures/svgs/recursive/bear.svg"),
            Path::new("./tests/fixtures/svgs/recursive")
        )
        .unwrap(),
        "bear"
    );
}

#[test]
fn sprite_name_works_with_nested_files() {
    assert_eq!(
        sprite_name(
            Path::new("./tests/fixtures/svgs/recursive/bear.svg"),
            Path::new("./tests/fixtures/svgs")
        )
        .unwrap(),
        "recursive/bear"
    );
}

#[test]
fn sprite_name_works_with_deeply_nested_files() {
    assert_eq!(
        sprite_name(
            Path::new("./tests/fixtures/svgs/recursive/bear.svg"),
            Path::new("./tests")
        )
        .unwrap(),
        "fixtures/svgs/recursive/bear"
    );
}

#[test]
fn sprite_name_returns_ok_for_non_existent_path() {
    assert_eq!(
        sprite_name(Path::new("./does_not_exist.svg"), Path::new("./")).unwrap(),
        "does_not_exist"
    );
}

#[test]
fn sprite_name_returns_error_when_path_is_empty() {
    assert_matches!(
        sprite_name(Path::new(""), Path::new("")),
        Err(SpreetError::IoError(_))
    );
}

#[test]
fn sprite_name_returns_error_for_non_existent_base_path() {
    assert_matches!(
        sprite_name(
            Path::new("./tests/fixtures/svgs/bicycle.svg"),
            Path::new("./tests/fixtures/foo"),
        ),
        Err(SpreetError::PathError(_))
    );
}

#[test]
fn sprite_name_returns_error_when_base_path_not_parent_of_path() {
    assert_matches!(
        sprite_name(
            Path::new("./tests/fixtures/svgs/bicycle.svg"),
            Path::new("./tests/fixtures/pngs/"),
        ),
        Err(SpreetError::PathError(_))
    );
}

#[test]
fn unstretchable_icon_has_no_metadata() {
    let path = Path::new("./tests/fixtures/svgs/bicycle.svg");
    let tree = load_svg(path).unwrap();
    let sprite = Sprite::new(tree, 1).unwrap();

    assert!(sprite.content_area().is_none());
    assert!(sprite.stretch_x_areas().is_none());
    assert!(sprite.stretch_y_areas().is_none());
}

#[test]
fn stretchable_icon_has_metadata() {
    let path = Path::new("./tests/fixtures/stretchable/cn-nths-expy-2-affinity.svg");
    let tree = load_svg(path).unwrap();
    let sprite = Sprite::new(tree, 1).unwrap();

    assert_eq!(
        sprite.content_area().unwrap(),
        Rect::from_ltrb(2.0, 5.0, 18.0, 18.0).unwrap()
    );
    assert_eq!(
        sprite.stretch_x_areas().unwrap(),
        [Rect::from_ltrb(4.0, 0.0, 16.0, 0.0).unwrap()]
    );
    assert_eq!(
        sprite.stretch_y_areas().unwrap(),
        [Rect::from_ltrb(0.0, 5.0, 0.0, 16.0).unwrap()]
    );
}

#[test]
fn stretchable_icons_can_use_stretch_shorthand() {
    let path = Path::new("./tests/fixtures/stretchable/cn-nths-expy-2-inkscape-plain.svg");
    let tree = load_svg(path).unwrap();
    let sprite = Sprite::new(tree, 1).unwrap();

    assert!(sprite.content_area().is_none());
    assert_eq!(
        sprite.stretch_x_areas().unwrap(),
        [Rect::from_ltrb(3.0, 5.0, 17.0, 17.0).unwrap()],
    );
    assert_eq!(
        sprite.stretch_y_areas().unwrap(),
        [Rect::from_ltrb(3.0, 5.0, 17.0, 17.0).unwrap()],
    );
}

#[test]
fn stretchable_icon_can_have_multiple_horizontal_stretch_zones() {
    let path = Path::new("./tests/fixtures/stretchable/ae-national-3-affinity.svg");
    let tree = load_svg(path).unwrap();
    let sprite = Sprite::new(tree, 1).unwrap();

    assert_eq!(
        sprite.stretch_x_areas().unwrap(),
        [
            Rect::from_ltrb(5.0, 5.0, 7.0, 5.0).unwrap(),
            Rect::from_ltrb(20.0, 5.0, 22.0, 5.0).unwrap(),
        ]
    );
}

#[test]
fn stretchable_icon_metadata_matches_pixel_ratio() {
    let path = Path::new("./tests/fixtures/stretchable/cn-nths-expy-2-affinity.svg");
    let tree = load_svg(path).unwrap();
    let sprite = Sprite::new(tree, 2).unwrap();

    assert_eq!(
        sprite.content_area().unwrap(),
        Rect::from_ltrb(4.0, 10.0, 36.0, 36.0).unwrap()
    );
    assert_eq!(
        sprite.stretch_x_areas().unwrap(),
        [Rect::from_ltrb(8.0, 0.0, 32.0, 0.0).unwrap()]
    );
    assert_eq!(
        sprite.stretch_y_areas().unwrap(),
        [Rect::from_ltrb(0.0, 10.0, 0.0, 32.0).unwrap()]
    );
}

#[test]
fn stretchable_icon_with_empty_metadata_is_ignored() {
    let svg = "<svg xmlns='http://www.w3.org/2000/svg'><path id='mapbox-content'/></svg>";
    let tree = Tree::from_str(svg, &Options::default()).unwrap();
    let sprite = Sprite::new(tree, 1).unwrap();

    assert!(sprite.content_area().is_none());
}

#[test]
fn stretchable_icon_with_invalid_metadata_is_ignored() {
    let svg = "<svg xmlns='http://www.w3.org/2000/svg'><path id='mapbox-content' d='foo'/></svg>";
    let tree = Tree::from_str(svg, &Options::default()).unwrap();
    let sprite = Sprite::new(tree, 1).unwrap();

    assert!(sprite.content_area().is_none());
}

#[test]
fn stretchable_icon_with_metadata_in_hidden_element_is_ignored() {
    let svg = "
    <svg xmlns='http://www.w3.org/2000/svg'>
        <path id='mapbox-content' d='M5,5l2,0' style='display:none'/>
    </svg>
    ";
    let tree = Tree::from_str(svg, &Options::default()).unwrap();
    let sprite = Sprite::new(tree, 1).unwrap();

    assert!(sprite.content_area().is_none());
}

#[test]
fn unique_sprite_matching_works() {
    let red = "
        <svg xmlns='http://www.w3.org/2000/svg' width='4' height='4'>
            <rect width='4' height='4' fill='#f00'/>
        </svg>
    ";
    let blue = "
        <svg xmlns='http://www.w3.org/2000/svg' width='4' height='4'>
            <rect width='4' height='4' fill='#00f'/>
        </svg>
    ";
    let mut sprites = BTreeMap::new();
    for (name, svg) in [("a", red), ("b", red), ("c", blue)] {
        let tree = Tree::from_str(svg, &Options::default()).unwrap();
        sprites.insert(name.to_string(), Sprite::new(tree, 1).unwrap());
    }

    let spritesheet = Spritesheet::build()
        .sprites(sprites)
        .make_unique()
        .generate()
        .unwrap();
    let index = spritesheet.get_index();

    assert_eq!(index.len(), 3);
    assert_eq!((index["a"].x, index["a"].y), (index["b"].x, index["b"].y));
    assert_ne!((index["a"].x, index["a"].y), (index["c"].x, index["c"].y));
}

// ---------------------------------------------------------------------------
// Sprite::from_png (PNG input support, scheme C5)
// ---------------------------------------------------------------------------

/// Encode raw pixels into a PNG file. `bytes_per_pixel` is 1 (gray), 2 (gray+alpha), 3 (rgb),
/// or 4 (rgba). `bit_depth` is 8 or 16.
fn write_png(
    path: &Path,
    width: u32,
    height: u32,
    bytes_per_pixel: usize,
    bit_depth: u16,
    pixels: &[u8],
) {
    let file = std::fs::File::create(path).unwrap();
    let mut encoder = png::Encoder::new(file, width, height);
    encoder.set_color(match bytes_per_pixel {
        1 => png::ColorType::Grayscale,
        2 => png::ColorType::GrayscaleAlpha,
        3 => png::ColorType::Rgb,
        4 => png::ColorType::Rgba,
        _ => panic!("unsupported bytes_per_pixel"),
    });
    encoder.set_depth(match bit_depth { 8 => png::BitDepth::Eight, 16 => png::BitDepth::Sixteen, _ => panic!("unsupported bit_depth"), });
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(pixels).unwrap();
}

#[test]
fn from_png_rgba_is_premultiplied() {
    let temp = assert_fs::TempDir::new().unwrap();
    let path = temp.join("icon.png");
    // A single semi-transparent red pixel: straight RGBA (255, 0, 0, 128).
    write_png(&path, 1, 1, 4, 8, &[255, 0, 0, 128]);

    let sprite = Sprite::from_png(&path, 1).unwrap();
    let pixmap = sprite.pixmap();
    assert_eq!((pixmap.width(), pixmap.height()), (1, 1));
    // tiny-skia stores premultiplied RGBA: (128, 0, 0, 128).
    let px = pixmap.pixels()[0];
    assert_eq!(
        (px.red(), px.green(), px.blue(), px.alpha()),
        (128, 0, 0, 128)
    );
}

#[test]
fn from_png_rgb_gets_opaque_alpha() {
    let temp = assert_fs::TempDir::new().unwrap();
    let path = temp.join("icon.png");
    write_png(&path, 2, 1, 3, 8, &[255, 128, 0, 0, 255, 255]);

    let sprite = Sprite::from_png(&path, 1).unwrap();
    let px = sprite.pixmap().pixels();
    assert_eq!(px.len(), 2);
    assert_eq!(
        (px[0].red(), px[0].green(), px[0].blue(), px[0].alpha()),
        (255, 128, 0, 255)
    );
    assert_eq!(
        (px[1].red(), px[1].green(), px[1].blue(), px[1].alpha()),
        (0, 255, 255, 255)
    );
}

#[test]
fn from_png_grayscale_and_palette_variants() {
    let temp = assert_fs::TempDir::new().unwrap();

    // Grayscale: single 8-bit channel per pixel.
    let gray_path = temp.join("gray.png");
    write_png(&gray_path, 1, 1, 1, 8, &[200]);
    let sprite = Sprite::from_png(&gray_path, 1).unwrap();
    let px = sprite.pixmap().pixels()[0];
    assert_eq!((px.red(), px.green(), px.blue(), px.alpha()), (200, 200, 200, 255));

    // Grayscale + alpha.
    let gray_alpha_path = temp.join("gray_alpha.png");
    write_png(&gray_alpha_path, 1, 1, 2, 8, &[100, 64]);
    let sprite = Sprite::from_png(&gray_alpha_path, 1).unwrap();
    let px = sprite.pixmap().pixels()[0];
    assert_eq!((px.red(), px.green(), px.blue(), px.alpha()), (25, 25, 25, 64));

    // 16-bit RGBA gets stripped to 8-bit.
    let deep_path = temp.join("deep.png");
    let mut pixels = Vec::new();
    for v in [0xFFFFu16, 0x0000u16, 0x8000u16, 0xFFFFu16] {
        pixels.extend_from_slice(&v.to_be_bytes());
    }
    write_png(&deep_path, 1, 1, 4, 16, &pixels);
    let sprite = Sprite::from_png(&deep_path, 1).unwrap();
    let px = sprite.pixmap().pixels()[0];
    assert_eq!((px.red(), px.green(), px.blue(), px.alpha()), (255, 0, 128, 255));

    // Palette (indexed) PNG with a tRNS chunk, as a checked-in fixture (generated with PIL:
    // 2x2, palette entry 0 = opaque red).
    let palette_path = Path::new("tests/fixtures/png_palette/palette.png");
    let sprite = Sprite::from_png(palette_path, 1).unwrap();
    assert_eq!(sprite.pixmap().width(), 2);
    assert_eq!(sprite.pixmap().height(), 2);
    let px = sprite.pixmap().pixels()[0];
    assert_eq!((px.red(), px.green(), px.blue(), px.alpha()), (255, 0, 0, 255));
}

#[test]
fn from_png_error_on_invalid_png() {
    let temp = assert_fs::TempDir::new().unwrap();
    let path = temp.join("broken.png");
    std::fs::write(&path, b"not a png").unwrap();
    assert!(matches!(
        Sprite::from_png(&path, 1),
        Err(SpreetError::PngDecodingError(_))
    ));
}

#[test]
fn from_png_sdf_flag_follows_path() {
    let temp = assert_fs::TempDir::new().unwrap();

    // Path contains `sdf` after position zero: create a sub-directory with `sdf` in its name so
    // the assertion is deterministic regardless of the temp dir suffix.
    let sdf_dir = temp.join("sdf-icons");
    std::fs::create_dir_all(&sdf_dir).unwrap();
    let sdf_path = sdf_dir.join("a.png");
    write_png(&sdf_path, 1, 1, 4, 8, &[0, 0, 0, 255]);
    let sprite = Sprite::from_png(&sdf_path, 1).unwrap();
    assert!(sprite.is_sdf());

    let plain_path = temp.join("a.png");
    write_png(&plain_path, 1, 1, 4, 8, &[0, 0, 0, 255]);
    let sprite = Sprite::from_png(&plain_path, 1).unwrap();
    assert!(!sprite.is_sdf());
}

#[test]
fn from_png_sprite_has_no_svg_metadata() {
    let temp = assert_fs::TempDir::new().unwrap();
    let path = temp.join("icon.png");
    write_png(&path, 1, 1, 4, 8, &[0, 0, 0, 255]);
    let sprite = Sprite::from_png(&path, 1).unwrap();

    assert!(sprite.tree().is_none());
    assert!(sprite.content_area().is_none());
    assert!(sprite.stretch_x_areas().is_none());
    assert!(sprite.stretch_y_areas().is_none());
}

#[test]
fn png_sprite_via_spritesheet_index_has_sdf_flag() {
    let temp = assert_fs::TempDir::new().unwrap();
    let sdf_dir = temp.join("sdf-icons");
    std::fs::create_dir_all(&sdf_dir).unwrap();
    let path = sdf_dir.join("icon.png");
    write_png(&path, 2, 2, 4, 8, &[0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255]);

    let mut sprites = BTreeMap::new();
    sprites.insert("icon".to_string(), Sprite::from_png(&path, 1).unwrap());
    let spritesheet = Spritesheet::build().sprites(sprites).generate().unwrap();
    let index = spritesheet.get_index();

    assert!(index["icon"].sdf);
}
