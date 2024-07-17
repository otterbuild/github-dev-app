//! Command to register a new GitHub App

use std::env::var;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use anyhow::{Context, Error};
use async_trait::async_trait;
use reqwest::Client;
use url::Url;

use crate::cli::Args;
use crate::Execute;

use super::app::App;
use super::env::save_to_env;
use super::server::start_background_web_server;
use super::RegisterArgs;

/// Register a new GitHub App
///
/// This command is used to register a new GitHub App from a manifest file. The manifest file must
/// be provided as an argument to the command. The command will parse the manifest, optionally
/// customize it, and then register the app with GitHub.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct RegisterCommand<'a> {
    /// The arguments for the command
    args: &'a RegisterArgs,
}

impl<'a> RegisterCommand<'a> {
    /// Create a new instance of the command
    pub fn new(args: &'a RegisterArgs) -> Self {
        Self { args }
    }

    /// Open the form that starts the registration process
    ///
    /// This method opens the form that starts the registration process in the user's default web
    /// browser. The form sends the manifest to register a new GitHub App, allowing the user to
    /// customize the name. After the user registers the app, they will be redirected back to the
    /// local web server to complete the registration process.
    fn open_registration_form(&self, addr: &SocketAddr) -> Result<(), Error> {
        // Skip opening the browser if running in CI
        if var("CI").is_ok() {
            return Ok(());
        }

        // Replace 0.0.0.0 with localhost to prevent warnings in the browser
        let host = replace_localhost(addr);

        open::that(format!("http://{}:{}", host, addr.port()))
            .context("failed to open browser to start registration process")
    }
}

#[async_trait]
impl<'a> Execute for RegisterCommand<'a> {
    async fn execute(&self, _global_args: &Args) -> Result<(), Error> {
        let (addr, mut receiver) = start_background_web_server(
            self.args.manifest(),
            self.args.github().clone(),
            self.args.port(),
        )
        .await?;

        // Open a browser to start the registration process
        self.open_registration_form(&addr)?;

        // Wait for the user to be redirected back to the local server with a temporary code
        let temporary_code = receiver
            .recv()
            .await
            .context("failed to receive temporary code from internal channel")?;

        // Exchange the temporary code for the app secrets
        let app = exchange_temporary_code(self.args.github(), &temporary_code).await?;

        // Save secrets and private key to the .env file
        save_to_env(&app)?;

        Ok(())
    }
}

/// Replace 0.0.0.0 with localhost
///
/// Modern browsers tend to handle `localhost` differently than an IP address. This function checks
/// if a socket address uses `0.0.0.0` and if so, replaces it with `localhost`.
fn replace_localhost(addr: &SocketAddr) -> String {
    if addr.ip() == IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)) {
        String::from("localhost")
    } else {
        addr.ip().to_string()
    }
}

/// Exchange a temporary code for the app secrets
///
/// This function exchanges a temporary code for the app secrets. The temporary code is provided by
/// GitHub after the user registers the app. The function sends the temporary code to GitHub and
/// receives the app's id, secrets, and private key in return.
async fn exchange_temporary_code(github: &Url, code: &str) -> Result<App, Error> {
    let url = github.join(&format!("app-manifests/{code}/conversions"))?;

    Client::new()
        .post(url)
        .send()
        .await
        .context("failed to convert temporary code")?
        .json()
        .await
        .context("failed to parse conversion response")
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn replace_localhost_with_local_ip() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 8080);

        let host = replace_localhost(&addr);

        assert_eq!("localhost", host);
    }

    #[test]
    fn replace_localhost_with_remote_ip() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4)), 8080);

        let host = replace_localhost(&addr);

        assert_eq!("1.2.3.4", host);
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<RegisterCommand>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<RegisterCommand>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<RegisterCommand>();
    }
}
