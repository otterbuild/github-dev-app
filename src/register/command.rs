//! Command to register a new GitHub App

use std::env::var;
use std::net::SocketAddr;
use std::time::Duration;

use anyhow::{Context, Error};
use async_trait::async_trait;
use tokio::time::sleep;

use crate::cli::Args;
use crate::register::server::start_background_web_server;
use crate::register::RegisterArgs;
use crate::Execute;

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

        open::that(format!("http://{}:{}", addr.ip(), addr.port()))
            .context("failed to open browser to start registration process")
    }
}

#[async_trait]
impl<'a> Execute for RegisterCommand<'a> {
    async fn execute(&self, _global_args: &Args) -> Result<(), Error> {
        let (addr, _receiver) =
            start_background_web_server(self.args.manifest(), self.args.port()).await?;

        // Open a browser to start the registration process
        self.open_registration_form(&addr)?;

        // Wait for browser to open
        sleep(Duration::from_secs(10)).await;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
