use std::process::Command;

use anyhow::Error;
use assert_cmd::prelude::*;

#[test]
fn prints_help() -> Result<(), Error> {
    let mut command = Command::cargo_bin("github-dev-app")?;

    command.arg("--help");

    command
        .assert()
        .success()
        .stdout(predicates::str::contains("Usage: github-dev-app"));

    Ok(())
}
