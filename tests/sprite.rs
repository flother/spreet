use std::path::Path;

use spreet::sprite::sprite_name;

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
