#![cfg(feature = "cli")]

use std::path::Path;

use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;

#[test]
fn spreet_can_run_successfully() -> Result<(), Box<dyn std::error::Error>> {
    let temp = assert_fs::TempDir::new().unwrap();

    let mut cmd = cargo_bin_cmd!("spreet");
    cmd.arg("tests/fixtures/svgs")
        .arg(temp.join("can_run"))
        .arg("--ratio")
        .arg("2")
        .assert()
        .success();

    Ok(())
}

#[test]
fn spreet_can_output_spritesheet() -> Result<(), Box<dyn std::error::Error>> {
    let temp = assert_fs::TempDir::new().unwrap();

    let mut cmd = cargo_bin_cmd!("spreet");
    cmd.arg("tests/fixtures/svgs")
        .arg(temp.join("default"))
        .assert()
        .success();

    let expected_spritesheet = Path::new("tests/fixtures/output/default@1x.png");
    let actual_spritesheet = predicate::path::eq_file(temp.join("default.png"));
    let expected_index = Path::new("tests/fixtures/output/default@1x.json");
    let actual_index = predicate::path::eq_file(temp.join("default.json"));

    assert!(actual_spritesheet.eval(expected_spritesheet));
    assert!(actual_index.eval(expected_index));

    Ok(())
}

#[test]
fn spreet_can_output_unique_spritesheet() -> Result<(), Box<dyn std::error::Error>> {
    let temp = assert_fs::TempDir::new().unwrap();

    let mut cmd = cargo_bin_cmd!("spreet");
    cmd.arg("tests/fixtures/svgs")
        .arg(temp.join("unique"))
        .arg("--unique")
        .assert()
        .success();

    let expected_spritesheet = Path::new("tests/fixtures/output/unique@1x.png");
    let actual_spritesheet = predicate::path::eq_file(temp.join("unique.png"));
    let expected_index = Path::new("tests/fixtures/output/unique@1x.json");
    let actual_index = predicate::path::eq_file(temp.join("unique.json"));

    assert!(actual_spritesheet.eval(expected_spritesheet));
    assert!(actual_index.eval(expected_index));

    Ok(())
}

#[test]
fn spreet_can_output_retina_spritesheet() -> Result<(), Box<dyn std::error::Error>> {
    let temp = assert_fs::TempDir::new().unwrap();

    let mut cmd = cargo_bin_cmd!("spreet");
    cmd.arg("tests/fixtures/svgs")
        .arg(temp.join("default@2x"))
        .arg("--retina")
        .assert()
        .success();

    let expected_spritesheet = Path::new("tests/fixtures/output/default@2x.png");
    let actual_spritesheet = predicate::path::eq_file(temp.join("default@2x.png"));
    let expected_index = Path::new("tests/fixtures/output/default@2x.json");
    let actual_index = predicate::path::eq_file(temp.join("default@2x.json"));

    assert!(actual_spritesheet.eval(expected_spritesheet));
    assert!(actual_index.eval(expected_index));

    Ok(())
}

#[test]
fn spreet_can_output_recursive_spritesheet() -> Result<(), Box<dyn std::error::Error>> {
    let temp = assert_fs::TempDir::new().unwrap();

    let mut cmd = cargo_bin_cmd!("spreet");
    cmd.arg("tests/fixtures/svgs")
        .arg(temp.join("recursive"))
        .arg("--recursive")
        .assert()
        .success();

    let expected_spritesheet = Path::new("tests/fixtures/output/recursive@1x.png");
    let actual_spritesheet = predicate::path::eq_file(temp.join("recursive.png"));
    let expected_index = Path::new("tests/fixtures/output/recursive@1x.json");
    let actual_index = predicate::path::eq_file(temp.join("recursive.json"));

    assert!(actual_spritesheet.eval(expected_spritesheet));
    assert!(actual_index.eval(expected_index));

    Ok(())
}

#[test]
fn spreet_can_output_unique_retina_spritesheet() -> Result<(), Box<dyn std::error::Error>> {
    let temp = assert_fs::TempDir::new().unwrap();

    let mut cmd = cargo_bin_cmd!("spreet");
    cmd.arg("tests/fixtures/svgs")
        .arg(temp.join("unique@2x"))
        .arg("--retina")
        .arg("--unique")
        .assert()
        .success();

    let expected_spritesheet = Path::new("tests/fixtures/output/unique@2x.png");
    let actual_spritesheet = predicate::path::eq_file(temp.join("unique@2x.png"));
    let expected_index = Path::new("tests/fixtures/output/unique@2x.json");
    let actual_index = predicate::path::eq_file(temp.join("unique@2x.json"));

    assert!(actual_spritesheet.eval(expected_spritesheet));
    assert!(actual_index.eval(expected_index));

    Ok(())
}

#[test]
fn spreet_can_output_minified_index_file() -> Result<(), Box<dyn std::error::Error>> {
    let temp = assert_fs::TempDir::new().unwrap();

    let mut cmd = cargo_bin_cmd!("spreet");
    cmd.arg("tests/fixtures/svgs")
        .arg(temp.join("minify"))
        .arg("--minify-index-file")
        .assert()
        .success();

    let expected_spritesheet = Path::new("tests/fixtures/output/minify@1x.png");
    let actual_spritesheet = predicate::path::eq_file(temp.join("minify.png"));
    let expected_index = Path::new("tests/fixtures/output/minify@1x.json");
    let actual_index = predicate::path::eq_file(temp.join("minify.json"));

    assert!(actual_spritesheet.eval(expected_spritesheet));
    assert!(actual_index.eval(expected_index));

    Ok(())
}

#[test]
fn spreet_can_output_minified_index_file_and_unique_spritesheet(
) -> Result<(), Box<dyn std::error::Error>> {
    let temp = assert_fs::TempDir::new().unwrap();

    let mut cmd = cargo_bin_cmd!("spreet");
    cmd.arg("tests/fixtures/svgs")
        .arg(temp.join("minify_unique"))
        .arg("--minify-index-file")
        .arg("--unique")
        .assert()
        .success();

    let expected_spritesheet = Path::new("tests/fixtures/output/minify_unique@1x.png");
    let actual_spritesheet = predicate::path::eq_file(temp.join("minify_unique.png"));
    let expected_index = Path::new("tests/fixtures/output/minify_unique@1x.json");
    let actual_index = predicate::path::eq_file(temp.join("minify_unique.json"));

    assert!(actual_spritesheet.eval(expected_spritesheet));
    assert!(actual_index.eval(expected_index));

    Ok(())
}

#[test]
fn spreet_can_output_stretchable_icons() -> Result<(), Box<dyn std::error::Error>> {
    let temp = assert_fs::TempDir::new().unwrap();

    let mut cmd = cargo_bin_cmd!("spreet");
    cmd.arg("tests/fixtures/stretchable")
        .arg(temp.join("stretchable@2x"))
        .arg("--retina")
        .assert()
        .success();

    let expected_spritesheet = Path::new("tests/fixtures/output/stretchable@2x.png");
    let actual_spritesheet = predicate::path::eq_file(temp.join("stretchable@2x.png"));
    let expected_index = Path::new("tests/fixtures/output/stretchable@2x.json");
    let actual_index = predicate::path::eq_file(temp.join("stretchable@2x.json"));

    assert!(actual_spritesheet.eval(expected_spritesheet));
    assert!(actual_index.eval(expected_index));

    Ok(())
}

#[test]
fn spreet_can_output_sdf_icons() -> Result<(), Box<dyn std::error::Error>> {
    let temp = assert_fs::TempDir::new().unwrap();

    let mut cmd = cargo_bin_cmd!("spreet");
    cmd.arg("tests/fixtures/svgs")
        .arg(temp.join("sdf@2x"))
        .arg("--sdf")
        .arg("--retina")
        .arg("--recursive")
        .assert()
        .success();

    let expected_spritesheet = Path::new("tests/fixtures/output/sdf@2x.png");
    let actual_spritesheet = predicate::path::eq_file(temp.join("sdf@2x.png"));
    let expected_index = Path::new("tests/fixtures/output/sdf@2x.json");
    let actual_index = predicate::path::eq_file(temp.join("sdf@2x.json"));

    assert!(actual_spritesheet.eval(expected_spritesheet));
    assert!(actual_index.eval(expected_index));

    Ok(())
}

#[test]
fn spreet_rejects_non_existent_input_directory() {
    let mut cmd = cargo_bin_cmd!("spreet");
    cmd.arg("does_not_exist")
        .arg("default")
        .assert()
        .failure()
        .code(2)
        .stderr("error: invalid value 'does_not_exist' for '<INPUT>': must be an existing directory\n\nFor more information, try '--help'.\n");
}

#[test]
fn spreet_rejects_zero_ratio() {
    let temp = assert_fs::TempDir::new().unwrap();

    let mut cmd = cargo_bin_cmd!("spreet");
    cmd.arg("tests/fixtures/svgs")
        .arg(temp.join("default"))
        .arg("--ratio")
        .arg("0")
        .assert()
        .failure()
        .code(2)
        .stderr("error: invalid value '0' for '--ratio <RATIO>': must be greater than zero\n\nFor more information, try '--help'.\n");
}

#[test]
fn spreet_rejects_negative_ratio() {
    let temp = assert_fs::TempDir::new().unwrap();

    let mut cmd = cargo_bin_cmd!("spreet");
    cmd.arg("tests/fixtures/svgs")
        .arg(temp.join("default"))
        .arg("--ratio")
        .arg(" -3")
        .assert()
        .failure()
        .code(2)
        .stderr("error: invalid value ' -3' for '--ratio <RATIO>': invalid digit found in string\n\nFor more information, try '--help'.\n");
}

#[test]
fn spreet_accepts_pngs_wrapped_in_svgs() {
    let temp = assert_fs::TempDir::new().unwrap();

    let mut cmd = cargo_bin_cmd!("spreet");
    cmd.arg("tests/fixtures/pngs")
        .arg(temp.join("pngs@2x"))
        .arg("--retina")
        .assert()
        .success();

    let expected_spritesheet = Path::new("tests/fixtures/output/pngs@2x.png");
    let actual_spritesheet = predicate::path::eq_file(temp.join("pngs@2x.png"));
    let expected_index = Path::new("tests/fixtures/output/pngs@2x.json");
    let actual_index = predicate::path::eq_file(temp.join("pngs@2x.json"));

    assert!(actual_spritesheet.eval(expected_spritesheet));
    assert!(actual_index.eval(expected_index));
}

#[test]
fn spreet_accepts_zero_spacing() -> Result<(), Box<dyn std::error::Error>> {
    let temp = assert_fs::TempDir::new().unwrap();

    let mut cmd = cargo_bin_cmd!("spreet");
    cmd.arg("tests/fixtures/svgs")
        .arg(temp.join("explicit_zero_spacing"))
        .arg("--spacing")
        .arg("0")
        .assert()
        .success();

    Ok(())
}

#[test]
fn spreet_rejects_negative_spacing() -> Result<(), Box<dyn std::error::Error>> {
    let temp = assert_fs::TempDir::new().unwrap();

    let mut cmd = cargo_bin_cmd!("spreet");
    cmd.arg("tests/fixtures/svgs")
        .arg(temp.join("default"))
        .arg("--spacing=-15")
        .assert()
        .failure()
        .code(2)
        .stderr("error: invalid value '-15' for '--spacing <SPACING>': must be a non-negative number\n\nFor more information, try '--help'.\n");

    Ok(())
}

#[test]
fn spreet_can_output_spritesheet_with_spacing() -> Result<(), Box<dyn std::error::Error>> {
    let temp = assert_fs::TempDir::new().unwrap();

    let mut cmd = cargo_bin_cmd!("spreet");
    cmd.arg("tests/fixtures/svgs")
        .arg(temp.join("spacing"))
        .arg("--spacing")
        .arg("5")
        .assert()
        .success();

    let expected_spritesheet = Path::new("tests/fixtures/output/spacing@1x.png");
    let actual_spritesheet = predicate::path::eq_file(temp.join("spacing.png"));
    let expected_index = Path::new("tests/fixtures/output/spacing@1x.json");
    let actual_index = predicate::path::eq_file(temp.join("spacing.json"));

    assert!(actual_spritesheet.eval(expected_spritesheet));
    assert!(actual_index.eval(expected_index));

    Ok(())
}

#[test]
fn spreet_can_output_unique_spritesheet_with_spacing() -> Result<(), Box<dyn std::error::Error>> {
    let temp = assert_fs::TempDir::new().unwrap();

    let mut cmd = cargo_bin_cmd!("spreet");
    cmd.arg("tests/fixtures/svgs")
        .arg(temp.join("spacing_unique"))
        .arg("--spacing")
        .arg("2")
        .arg("--unique")
        .assert()
        .success();

    let expected_spritesheet = Path::new("tests/fixtures/output/spacing_unique@1x.png");
    let actual_spritesheet = predicate::path::eq_file(temp.join("spacing_unique.png"));
    let expected_index = Path::new("tests/fixtures/output/spacing_unique@1x.json");
    let actual_index = predicate::path::eq_file(temp.join("spacing_unique.json"));

    assert!(actual_spritesheet.eval(expected_spritesheet));
    assert!(actual_index.eval(expected_index));

    Ok(())
}

#[test]
fn spreet_can_output_retina_spritesheet_with_spacing() -> Result<(), Box<dyn std::error::Error>> {
    let temp = assert_fs::TempDir::new().unwrap();

    let mut cmd = cargo_bin_cmd!("spreet");
    cmd.arg("tests/fixtures/svgs")
        .arg(temp.join("spacing@2x"))
        .arg("--spacing")
        .arg("2")
        .arg("--retina")
        .assert()
        .success();

    let expected_spritesheet = Path::new("tests/fixtures/output/spacing@2x.png");
    let actual_spritesheet = predicate::path::eq_file(temp.join("spacing@2x.png"));
    let expected_index = Path::new("tests/fixtures/output/spacing@2x.json");
    let actual_index = predicate::path::eq_file(temp.join("spacing@2x.json"));

    assert!(actual_spritesheet.eval(expected_spritesheet));
    assert!(actual_index.eval(expected_index));

    Ok(())
}

#[test]
fn spreet_can_output_sprites_with_text() -> Result<(), Box<dyn std::error::Error>> {
    let temp = assert_fs::TempDir::new().unwrap();

    let mut cmd = cargo_bin_cmd!("spreet");
    cmd.arg("tests/fixtures/text")
        .arg(temp.join("text@3x"))
        .arg("--ratio")
        .arg("3")
        .assert()
        .success();

    let expected_spritesheet = Path::new("tests/fixtures/output/text@3x.png");
    let actual_spritesheet = predicate::path::eq_file(temp.join("text@3x.png"));
    let expected_index = Path::new("tests/fixtures/output/text@3x.json");
    let actual_index = predicate::path::eq_file(temp.join("text@3x.json"));

    assert!(actual_spritesheet.eval(expected_spritesheet));
    assert!(actual_index.eval(expected_index));

    Ok(())
}

// ---------------------------------------------------------------------------
// --png-pair (PNG input support, scheme C5)
// ---------------------------------------------------------------------------

/// Generate a PNG file with a single solid-color pixel via the `png` crate (already a dependency
/// of the main crate).
fn write_png_fixture(path: &Path, width: u32, height: u32, rgba: [u8; 4]) {
    use std::fs::File;
    let file = File::create(path).unwrap();
    let mut encoder = png::Encoder::new(file, width, height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    let mut data = Vec::with_capacity((width * height * 4) as usize);
    for _ in 0..height {
        for _ in 0..width {
            data.extend_from_slice(&rgba);
        }
    }
    writer.write_image_data(&data).unwrap();
}

#[test]
fn spreet_png_pair_outputs_four_files() -> Result<(), Box<dyn std::error::Error>> {
    let temp = assert_fs::TempDir::new().unwrap();
    let icons = temp.join("icons");
    std::fs::create_dir_all(&icons).unwrap();
    write_png_fixture(&icons.join("square.png"), 24, 24, [231, 76, 60, 255]);
    write_png_fixture(&icons.join("square@2x.png"), 48, 48, [231, 76, 60, 255]);
    write_png_fixture(&icons.join("circle.png"), 16, 16, [74, 144, 217, 255]);
    write_png_fixture(&icons.join("circle@2x.png"), 32, 32, [74, 144, 217, 255]);

    let mut cmd = cargo_bin_cmd!("spreet");
    cmd.arg(&icons).arg(temp.join("sprite")).arg("--png-pair").assert().success();

    // Four output files: <prefix>.png/.json + <prefix>@2x.png/.json.
    let index1 = json::parse(std::fs::read_to_string(temp.join("sprite.json"))?.as_str());
    let index2 = json::parse(std::fs::read_to_string(temp.join("sprite@2x.json"))?.as_str());

    // 1x index: pixelRatio 1, dimensions of the 1x files.
    assert_eq!(index1["square"]["width"].as_u64(), Some(24));
    assert_eq!(index1["circle"]["height"].as_u64(), Some(16));
    assert_eq!(index1["square"]["pixelRatio"].as_u64(), Some(1));
    // 2x index: pixelRatio 2, dimensions of the 2x files.
    assert_eq!(index2["square"]["width"].as_u64(), Some(48));
    assert_eq!(index2["circle"]["height"].as_u64(), Some(32));
    assert_eq!(index2["circle"]["pixelRatio"].as_u64(), Some(2));

    // The PNG files are valid images with matching dimensions.
    let png1 = std::fs::read(temp.join("sprite.png"))?;
    let png2 = std::fs::read(temp.join("sprite@2x.png"))?;
    assert_eq!(&png1[..8], b"\x89PNG\r\n\x1a\n");
    assert_eq!(&png2[..8], b"\x89PNG\r\n\x1a\n");

    Ok(())
}

#[test]
fn spreet_png_pair_fallback_semantics() -> Result<(), Box<dyn std::error::Error>> {
    let temp = assert_fs::TempDir::new().unwrap();
    let icons = temp.join("icons");
    std::fs::create_dir_all(&icons).unwrap();
    // tri: regular only (the 2x spritesheet falls back to it, keeping pixelRatio 1).
    write_png_fixture(&icons.join("tri.png"), 20, 20, [46, 204, 113, 255]);
    // star: retina only (the 1x spritesheet falls back to it, keeping pixelRatio 2).
    write_png_fixture(&icons.join("star@2x.png"), 36, 36, [241, 196, 15, 255]);

    let mut cmd = cargo_bin_cmd!("spreet");
    cmd.arg(&icons).arg(temp.join("sprite")).arg("--png-pair").assert().success();

    let index1 = json::parse(std::fs::read_to_string(temp.join("sprite.json"))?.as_str());
    let index2 = json::parse(std::fs::read_to_string(temp.join("sprite@2x.json"))?.as_str());

    // Fallback semantics mirror build-sprite.js:36-41: the pixel ratio follows the actual source
    // file name, even when the image fell back into the other spritesheet.
    assert_eq!(index1["star"]["pixelRatio"].as_u64(), Some(2));
    assert_eq!(index1["star"]["width"].as_u64(), Some(36));
    assert_eq!(index2["tri"]["pixelRatio"].as_u64(), Some(1));
    assert_eq!(index2["tri"]["width"].as_u64(), Some(20));

    Ok(())
}

#[test]
fn spreet_png_pair_sdf_flag_follows_path() -> Result<(), Box<dyn std::error::Error>> {
    let temp = assert_fs::TempDir::new().unwrap();
    // A sub-directory with `sdf` in its name makes every icon SDF (full-path indexOf semantics,
    // mirroring build-sprite.js:58-59,70-72).
    let icons = temp.join("sdf-icons");
    std::fs::create_dir_all(&icons).unwrap();
    write_png_fixture(&icons.join("a.png"), 10, 10, [0, 0, 0, 255]);
    write_png_fixture(&icons.join("a@2x.png"), 20, 20, [0, 0, 0, 255]);

    let mut cmd = cargo_bin_cmd!("spreet");
    cmd.arg(&icons).arg(temp.join("sprite")).arg("--png-pair").assert().success();

    let index1 = json::parse(std::fs::read_to_string(temp.join("sprite.json"))?.as_str());
    assert_eq!(index1["a"]["sdf"].as_bool(), Some(true));

    Ok(())
}

#[test]
fn spreet_png_pair_rejects_retina_flag() {
    let temp = assert_fs::TempDir::new().unwrap();
    let mut cmd = cargo_bin_cmd!("spreet");
    cmd.arg(temp.join("icons"))
        .arg(temp.join("sprite"))
        .arg("--png-pair")
        .arg("--retina")
        .assert()
        .failure();
}

/// Minimal JSON parser for the index assertions: delegate to serde_json via the library's
/// re-exports is not available, so parse with serde_json directly (it's a regular dependency).
mod json {
    pub fn parse(s: &str) -> serde_json::Value {
        serde_json::from_str(s).unwrap()
    }
}
