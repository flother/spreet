use std::path::Path;

use assert_matches::assert_matches;
use resvg::usvg::{Options, Rect, Tree};
use spreet::{load_svg, sprite_name, SpreetError, Sprite};

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
