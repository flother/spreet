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
fn spreet_rejects_non_existent_input_directory() {
    let mut cmd = Command::cargo_bin("spreet").unwrap();
    cmd.arg("does_not_exist")
        .arg("default")
        .assert()
        .failure()
        .code(2)
        .stderr("error: Invalid value 'does_not_exist' for '<INPUT>': must be an existing directory\n\nFor more information try '--help'\n");
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
        .stderr("error: Invalid value '0' for '--ratio <RATIO>': must be greater than one\n\nFor more information try '--help'\n");
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
        .stderr("error: Invalid value ' -3' for '--ratio <RATIO>': invalid digit found in string\n\nFor more information try '--help'\n");
}
