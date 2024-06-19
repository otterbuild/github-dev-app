use std::process::Command;

use anyhow::Error;
use assert_cmd::prelude::*;
use tempfile::NamedTempFile;

#[test]
fn prints_manifest() -> Result<(), Error> {
    let mut command = Command::cargo_bin("github-dev-app")?;

    let manifest = NamedTempFile::new()?;
    std::fs::write(manifest.path(), r#"{"url":"http://localhost"}"#)?;

    command.arg("register").arg(manifest.path());

    command
        .assert()
        .success()
        .stdout(predicates::str::contains("http://localhost"));

    Ok(())
}
