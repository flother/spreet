use std::path::Path;

use assert_matches::assert_matches;
use spreet::error::Error;
use spreet::fs::get_svg_input_paths;

#[test]
fn get_svg_input_paths_returns_non_recursive_results() {
    let mut input_paths = get_svg_input_paths(Path::new("tests/fixtures/svgs"), false).unwrap();
    input_paths.sort();
    assert_eq!(
        input_paths,
        vec![
            Path::new("tests/fixtures/svgs/another_bicycle.svg"),
            Path::new("tests/fixtures/svgs/bicycle.svg"),
            Path::new("tests/fixtures/svgs/circle.svg"),
        ]
    );
}

#[test]
fn get_svg_input_paths_returns_recursive_results() {
    let mut input_paths = get_svg_input_paths(Path::new("tests/fixtures/svgs"), true).unwrap();
    input_paths.sort();
    assert_eq!(
        input_paths,
        vec![
            Path::new("tests/fixtures/svgs/another_bicycle.svg"),
            Path::new("tests/fixtures/svgs/bicycle.svg"),
            Path::new("tests/fixtures/svgs/circle.svg"),
            Path::new("tests/fixtures/svgs/recursive/bear.svg"),
        ]
    );
}

#[test]
fn get_svg_input_paths_returns_error_when_path_does_not_exist() {
    assert_matches!(
        get_svg_input_paths(Path::new("fake"), false),
        Err(Error::IoError(_))
    );
}

#[test]
fn get_svg_input_paths_returns_error_when_path_is_file() {
    assert_matches!(
        get_svg_input_paths(Path::new("tests/fixtures/svgs/bicycle.svg"), false),
        Err(Error::IoError(_))
    );
}
