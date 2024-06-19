//! `github-dev-app`
//!
//! Create and manage a GitHub App for local development
//!
//! This command-line tool can be used to create and manage a GitHub App for local development. It
//! provides a simple way to register a new GitHub App from a manifest, add the app's secrets to the
//! .env file, and update the app when the manifest changes.

#![warn(missing_docs)]

use clap::Parser;

use crate::cli::Args;

mod cli;

fn main() {
    let _args = Args::parse();
}
