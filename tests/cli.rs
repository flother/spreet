use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn spreet_can_run_successfully() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("spreet")?;

    cmd.arg("tests/svgs")
        .arg("test_output@2x")
        .arg("--ratio")
        .arg("2");
    cmd.assert().success();

    Ok(())
}
