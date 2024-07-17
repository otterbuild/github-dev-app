use std::process::Command;
use std::time::Duration;

use anyhow::Error;
use assert_cmd::prelude::*;
use indoc::indoc;
use mockito::Server;
use port_check::is_port_reachable;
use pretty_assertions::assert_eq;
use reqwest::Client;
use tempfile::{tempdir, NamedTempFile};
use tokio::time::sleep;

const TEMPORARY_CODE: &str = "otters-are-the-cutest";

#[tokio::test]
async fn saves_private_key_and_secrets() -> Result<(), Error> {
    let mut command = Command::cargo_bin("github-dev-app")?;

    // Execute command in temporary directory
    let temp_dir = tempdir()?;
    command.current_dir(temp_dir.path());

    // Create a temporary manifest file
    let manifest = NamedTempFile::new_in(temp_dir.path())?;
    std::fs::write(manifest.path(), r#"{"url":"http://localhost"}"#)?;

    // Mock GitHub's API
    let mut server = Server::new_async().await;
    let server_url = server.url();
    let mock = server
        .mock(
            "POST",
            format!("/app-manifests/{TEMPORARY_CODE}/conversions").as_str(),
        )
        .with_body(indoc! {r#"
            {
              "id": 1,
              "name": "github-dev-app",
              "client_id": "Iv1.8a61f9b3a7aba766",
              "client_secret": "1726be1638095a19edd134c77bde3aa2ece1e5d8",
              "webhook_secret": "e340154128314309424b7c8e90325147d99fdafa",
              "pem": "-----BEGIN RSA PRIVATE KEY-----\nMIIEowIBAAKCAQEAuEPzOUE+kiEH1WLiMeBytTEF856j0hOVcSUSUkZxKvqczkWM\n9vo1gDyC7ZXhdH9fKh32aapba3RSsp4ke+giSmYTk2mGR538ShSDxh0OgpJmjiKP\nX0Bj4j5sFqfXuCtl9SkH4iueivv4R53ktqM+n6hk98l6hRwC39GVIblAh2lEM4L/\n6WvYwuQXPMM5OG2Ryh2tDZ1WS5RKfgq+9ksNJ5Q9UtqtqHkO+E63N5OK9sbzpUUm\noNaOl3udTlZD3A8iqwMPVxH4SxgATBPAc+bmjk6BMJ0qIzDcVGTrqrzUiywCTLma\nszdk8GjzXtPDmuBgNn+o6s02qVGpyydgEuqmTQIDAQABAoIBACL6AvkjQVVLn8kJ\ndBYznJJ4M8ECo+YEgaFwgAHODT0zRQCCgzd+Vxl4YwHmKV2Lr+y2s0drZt8GvYva\nKOK8NYYZyi15IlwFyRXmvvykF1UBpSXluYFDH7KaVroWMgRreHcIys5LqVSIb6Bo\ngDmK0yBLPp8qR29s2b7ScZRtLaqGJiX+j55rNzrZwxHkxFHyG9OG+u9IsBElcKCP\nkYCVE8ZdYexfnKOZbgn2kZB9qu0T/Mdvki8yk3I2bI6xYO24oQmhnT36qnqWoCBX\nNuCNsBQgpYZeZET8mEAUmo9d+ABmIHIvSs005agK8xRaP4+6jYgy6WwoejJRF5yd\nNBuF7aECgYEA50nZ4FiZYV0vcJDxFYeY3kYOvVuKn8OyW+2rg7JIQTremIjv8FkE\nZnwuF9ZRxgqLxUIfKKfzp/5l5LrycNoj2YKfHKnRejxRWXqG+ZETfxxlmlRns0QG\nJ4+BYL0CoanDSeA4fuyn4Bv7cy/03TDhfg/Uq0Aeg+hhcPE/vx3ebPsCgYEAy/Pv\neDLssOSdeyIxf0Brtocg6aPXIVaLdus+bXmLg77rJIFytAZmTTW8SkkSczWtucI3\nFI1I6sei/8FdPzAl62/JDdlf7Wd9K7JIotY4TzT7Tm7QU7xpfLLYIP1bOFjN81rk\n77oOD4LsXcosB/U6s1blPJMZ6AlO2EKs10UuR1cCgYBipzuJ2ADEaOz9RLWwi0AH\nPza2Sj+c2epQD9ZivD7Zo/Sid3ZwvGeGF13JyR7kLEdmAkgsHUdu1rI7mAolXMaB\n1pdrsHureeLxGbRM6za3tzMXWv1Il7FQWoPC8ZwXvMOR1VQDv4nzq7vbbA8z8c+c\n57+8tALQHOTDOgQIzwK61QKBgERGVc0EJy4Uag+VY8J4m1ZQKBluqo7TfP6DQ7O8\nM5MX73maB/7yAX8pVO39RjrhJlYACRZNMbK+v/ckEQYdJSSKmGCVe0JrGYDuPtic\nI9+IGfSorf7KHPoMmMN6bPYQ7Gjh7a++tgRFTMEc8956Hnt4xGahy9NcglNtBpVN\n6G8jAoGBAMCh028pdzJa/xeBHLLaVB2sc0Fe7993WlsPmnVE779dAz7qMscOtXJK\nfgtriltLSSD6rTA9hUAsL/X62rY0wdXuNdijjBb/qvrx7CAV6i37NK1CjABNjsfG\nZM372Ac6zc1EqSrid2IjET1YqyIW2KGLI1R2xbQc98UGlt48OdWu\n-----END RSA PRIVATE KEY-----\n"
            }
        "#})
        .create();

    // Execute the register command
    let mut process_handle = command
        .arg("register")
        .arg(manifest.path())
        .arg("--github")
        .arg(server_url)
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
        .get("http://localhost:64001/callback?code=otters-are-the-cutest")
        .send()
        .await
        .expect("failed to send temporary code");

    let exit_status = process_handle.wait().expect("failed to wait for command");

    // Assert that the mock was called
    mock.assert();

    // Assert that the command was successful
    assert!(exit_status.success());

    // Assert that the .env file contains the expected values
    let env = std::fs::read_to_string(temp_dir.path().join(".env"))?;
    assert_eq!(
        indoc! {r#"
        GITHUB_APP_ID=1
        GITHUB_APP_NAME=github-dev-app
        GITHUB_CLIENT_ID="Iv1.8a61f9b3a7aba766"
        GITHUB_CLIENT_SECRET=1726be1638095a19edd134c77bde3aa2ece1e5d8
        GITHUB_WEBHOOK_SECRET=e340154128314309424b7c8e90325147d99fdafa
        GITHUB_PRIVATE_KEY="-----BEGIN RSA PRIVATE KEY-----\nMIIEowIBAAKCAQEAuEPzOUE+kiEH1WLiMeBytTEF856j0hOVcSUSUkZxKvqczkWM\n9vo1gDyC7ZXhdH9fKh32aapba3RSsp4ke+giSmYTk2mGR538ShSDxh0OgpJmjiKP\nX0Bj4j5sFqfXuCtl9SkH4iueivv4R53ktqM+n6hk98l6hRwC39GVIblAh2lEM4L/\n6WvYwuQXPMM5OG2Ryh2tDZ1WS5RKfgq+9ksNJ5Q9UtqtqHkO+E63N5OK9sbzpUUm\noNaOl3udTlZD3A8iqwMPVxH4SxgATBPAc+bmjk6BMJ0qIzDcVGTrqrzUiywCTLma\nszdk8GjzXtPDmuBgNn+o6s02qVGpyydgEuqmTQIDAQABAoIBACL6AvkjQVVLn8kJ\ndBYznJJ4M8ECo+YEgaFwgAHODT0zRQCCgzd+Vxl4YwHmKV2Lr+y2s0drZt8GvYva\nKOK8NYYZyi15IlwFyRXmvvykF1UBpSXluYFDH7KaVroWMgRreHcIys5LqVSIb6Bo\ngDmK0yBLPp8qR29s2b7ScZRtLaqGJiX+j55rNzrZwxHkxFHyG9OG+u9IsBElcKCP\nkYCVE8ZdYexfnKOZbgn2kZB9qu0T/Mdvki8yk3I2bI6xYO24oQmhnT36qnqWoCBX\nNuCNsBQgpYZeZET8mEAUmo9d+ABmIHIvSs005agK8xRaP4+6jYgy6WwoejJRF5yd\nNBuF7aECgYEA50nZ4FiZYV0vcJDxFYeY3kYOvVuKn8OyW+2rg7JIQTremIjv8FkE\nZnwuF9ZRxgqLxUIfKKfzp/5l5LrycNoj2YKfHKnRejxRWXqG+ZETfxxlmlRns0QG\nJ4+BYL0CoanDSeA4fuyn4Bv7cy/03TDhfg/Uq0Aeg+hhcPE/vx3ebPsCgYEAy/Pv\neDLssOSdeyIxf0Brtocg6aPXIVaLdus+bXmLg77rJIFytAZmTTW8SkkSczWtucI3\nFI1I6sei/8FdPzAl62/JDdlf7Wd9K7JIotY4TzT7Tm7QU7xpfLLYIP1bOFjN81rk\n77oOD4LsXcosB/U6s1blPJMZ6AlO2EKs10UuR1cCgYBipzuJ2ADEaOz9RLWwi0AH\nPza2Sj+c2epQD9ZivD7Zo/Sid3ZwvGeGF13JyR7kLEdmAkgsHUdu1rI7mAolXMaB\n1pdrsHureeLxGbRM6za3tzMXWv1Il7FQWoPC8ZwXvMOR1VQDv4nzq7vbbA8z8c+c\n57+8tALQHOTDOgQIzwK61QKBgERGVc0EJy4Uag+VY8J4m1ZQKBluqo7TfP6DQ7O8\nM5MX73maB/7yAX8pVO39RjrhJlYACRZNMbK+v/ckEQYdJSSKmGCVe0JrGYDuPtic\nI9+IGfSorf7KHPoMmMN6bPYQ7Gjh7a++tgRFTMEc8956Hnt4xGahy9NcglNtBpVN\n6G8jAoGBAMCh028pdzJa/xeBHLLaVB2sc0Fe7993WlsPmnVE779dAz7qMscOtXJK\nfgtriltLSSD6rTA9hUAsL/X62rY0wdXuNdijjBb/qvrx7CAV6i37NK1CjABNjsfG\nZM372Ac6zc1EqSrid2IjET1YqyIW2KGLI1R2xbQc98UGlt48OdWu\n-----END RSA PRIVATE KEY-----\n"
    "#},
        env
    );

    Ok(())
}
