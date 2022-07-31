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
