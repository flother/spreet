use std::path::Path;

use resvg::usvg::{Options, Tree, TreeParsing};

use spreet::fs::load_svg;
use spreet::sprite::{sprite_name, Sprite};

#[test]
fn sprite_name_works_with_root_files() {
    assert_eq!(
        sprite_name(
            Path::new("./tests/fixtures/svgs/recursive/bear.svg"),
            Path::new("./tests/fixtures/svgs/recursive")
        ),
        "bear"
    );
}

#[test]
fn sprite_name_works_with_nested_files() {
    assert_eq!(
        sprite_name(
            Path::new("./tests/fixtures/svgs/recursive/bear.svg"),
            Path::new("./tests/fixtures/svgs")
        ),
        "recursive/bear"
    );
}

#[test]
fn sprite_name_works_with_deeply_nested_files() {
    assert_eq!(
        sprite_name(
            Path::new("./tests/fixtures/svgs/recursive/bear.svg"),
            Path::new("./tests")
        ),
        "fixtures/svgs/recursive/bear"
    );
}

#[test]
fn unstretchable_icon_has_no_metadata() {
    let path = Path::new("./tests/fixtures/svgs/bicycle.svg");
    let tree = load_svg(path).unwrap();
    let sprite = Sprite {
        tree,
        pixel_ratio: 1,
    };

    assert!(sprite.content_area().is_none());
    assert!(sprite.stretch_x_areas().is_none());
    assert!(sprite.stretch_y_areas().is_none());
}

#[test]
fn stretchable_icon_has_metadata() {
    let path = Path::new("./tests/fixtures/stretchable/cn-nths-expy-2-affinity.svg");
    let tree = load_svg(path).unwrap();
    let sprite = Sprite {
        tree,
        pixel_ratio: 1,
    };

    assert_eq!(sprite.content_area().unwrap(), [2.0, 5.0, 18.0, 18.0]);
    assert_eq!(sprite.stretch_x_areas().unwrap(), [[4.0, 16.0]]);
    assert_eq!(sprite.stretch_y_areas().unwrap(), [[5.0, 16.0]]);
}

#[test]
fn stretchable_icons_can_use_stretch_shorthand() {
    let path = Path::new("./tests/fixtures/stretchable/cn-nths-expy-2-inkscape-plain.svg");
    let tree = load_svg(path).unwrap();
    let sprite = Sprite {
        tree,
        pixel_ratio: 1,
    };

    assert!(sprite.content_area().is_none());
    assert_eq!(sprite.stretch_x_areas().unwrap(), [[3.0, 17.0]]);
    assert_eq!(sprite.stretch_y_areas().unwrap(), [[5.0, 17.0]]);
}

#[test]
fn stretchable_icon_can_have_multiple_horizontal_stretch_zones() {
    let path = Path::new("./tests/fixtures/stretchable/ae-national-3-affinity.svg");
    let tree = load_svg(path).unwrap();
    let sprite = Sprite {
        tree,
        pixel_ratio: 1,
    };

    assert_eq!(
        sprite.stretch_x_areas().unwrap(),
        [[5.0, 7.0], [20.0, 22.0]]
    );
}

#[test]
fn stretchable_icon_metadata_matches_pixel_ratio() {
    let path = Path::new("./tests/fixtures/stretchable/cn-nths-expy-2-affinity.svg");
    let tree = load_svg(path).unwrap();
    let sprite = Sprite {
        tree,
        pixel_ratio: 2,
    };

    assert_eq!(sprite.content_area().unwrap(), [4.0, 10.0, 36.0, 36.0]);
    assert_eq!(sprite.stretch_x_areas().unwrap(), [[8.0, 32.0]]);
    assert_eq!(sprite.stretch_y_areas().unwrap(), [[10.0, 32.0]]);
}

#[test]
fn stretchable_icon_with_empty_metadata_is_ignored() {
    let svg = "<svg xmlns='http://www.w3.org/2000/svg'><path id='mapbox-content'/></svg>";
    let tree = Tree::from_str(svg, &Options::default()).unwrap();
    let sprite = Sprite {
        tree,
        pixel_ratio: 1,
    };

    assert!(sprite.content_area().is_none());
}

#[test]
fn stretchable_icon_with_invalid_metadata_is_ignored() {
    let svg = "<svg xmlns='http://www.w3.org/2000/svg'><path id='mapbox-content' d='foo'/></svg>";
    let tree = Tree::from_str(svg, &Options::default()).unwrap();
    let sprite = Sprite {
        tree,
        pixel_ratio: 1,
    };

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
    let sprite = Sprite {
        tree,
        pixel_ratio: 1,
    };

    assert!(sprite.content_area().is_none());
}
