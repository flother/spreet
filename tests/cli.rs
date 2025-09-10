#![cfg(feature = "cli")]

use std::path::Path;
use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn spreet_can_run_successfully() -> Result<(), Box<dyn std::error::Error>> {
    let temp = assert_fs::TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("spreet")?;
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

    let mut cmd = Command::cargo_bin("spreet")?;
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

    let mut cmd = Command::cargo_bin("spreet")?;
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

    let mut cmd = Command::cargo_bin("spreet")?;
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

    let mut cmd = Command::cargo_bin("spreet")?;
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

    let mut cmd = Command::cargo_bin("spreet")?;
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

    let mut cmd = Command::cargo_bin("spreet")?;
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

    let mut cmd = Command::cargo_bin("spreet")?;
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

    let mut cmd = Command::cargo_bin("spreet")?;
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

    let mut cmd = Command::cargo_bin("spreet")?;
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
    let mut cmd = Command::cargo_bin("spreet").unwrap();
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

    let mut cmd = Command::cargo_bin("spreet").unwrap();
    cmd.arg("tests/fixtures/svgs")
        .arg(temp.join("default"))
        .arg("--ratio")
        .arg("0")
        .assert()
        .failure()
        .code(2)
        .stderr("error: invalid value '0' for '--ratio <RATIO>': must be greater than one\n\nFor more information, try '--help'.\n");
}

#[test]
fn spreet_rejects_negative_ratio() {
    let temp = assert_fs::TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("spreet").unwrap();
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

    let mut cmd = Command::cargo_bin("spreet").unwrap();
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

    let mut cmd = Command::cargo_bin("spreet")?;
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

    let mut cmd = Command::cargo_bin("spreet")?;
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

    let mut cmd = Command::cargo_bin("spreet")?;
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

    let mut cmd = Command::cargo_bin("spreet")?;
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

    let mut cmd = Command::cargo_bin("spreet")?;
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
