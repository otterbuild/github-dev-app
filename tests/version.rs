use std::process::Command;

use anyhow::Error;
use assert_cmd::prelude::*;

#[test]
fn prints_version() -> Result<(), Error> {
    let expected_output = format!("github-dev-app {}\n", env!("CARGO_PKG_VERSION"));
    let mut command = Command::cargo_bin("github-dev-app")?;

    command.arg("--version");

    command
        .assert()
        .success()
        .stdout(predicates::str::contains(expected_output));

    Ok(())
}
