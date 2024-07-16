use std::process::Command;
use std::time::Duration;

use anyhow::Error;
use assert_cmd::prelude::*;
use port_check::is_port_reachable;
use reqwest::Client;
use tempfile::NamedTempFile;
use tokio::time::sleep;

#[tokio::test]
async fn saves_private_key_and_secrets() -> Result<(), Error> {
    let mut command = Command::cargo_bin("github-dev-app")?;

    // Create a temporary manifest file
    let manifest = NamedTempFile::new()?;
    std::fs::write(manifest.path(), r#"{"url":"http://localhost"}"#)?;

    // Execute the register command
    let mut process_handle = command
        .arg("register")
        .arg(manifest.path())
        .arg("--port")
        .arg("64001")
        .spawn()
        .expect("failed to execute command");

    // Wait for server to launch
    for _ in 0..10 {
        if is_port_reachable("localhost:64001") {
            break;
        } else {
            sleep(Duration::from_secs(1)).await;
        }
    }

    // Send temporary code
    Client::new()
        .post("http://localhost:64001/callback?code=otters-are-the-cutest")
        .send()
        .await
        .expect("failed to send temporary code");

    let exit_status = process_handle.wait().expect("failed to wait for command");

    assert!(exit_status.success());

    Ok(())
}
