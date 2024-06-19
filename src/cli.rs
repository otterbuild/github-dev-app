//! Command-line arguments and options for `github-dev-app`
//!
//! This module defines the command-line interface for `github-dev-app`. It uses the `clap` crate to
//! parse command-line arguments and options. Documentation for the CLI is automatically generated
//! from the comments on the `Args` struct.

use clap::{Parser, Subcommand};

/// Create and manage a GitHub App for local development
///
/// This command-line tool can be used to create and manage a GitHub App for local development. It
/// provides a simple way to register a new GitHub App from a manifest, add the app's secrets to the
/// .env file, and update the app when the manifest changes.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Parser)]
#[command(version, about)]
pub struct Args {
    #[command(subcommand)]
    command: Command,
}

/// Top-level commands for `github-dev-app`
///
/// The `github-dev-app` command-line tool supports the top-level commands in this enum. Each
/// command has its own set of arguments and options that can be used to customize its behavior.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Subcommand)]
pub enum Command {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Args>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Args>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<Args>();
    }
}
