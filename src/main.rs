//! `github-dev-app`
//!
//! Create and manage a GitHub App for local development
//!
//! This command-line tool can be used to create and manage a GitHub App for local development. It
//! provides a simple way to register a new GitHub App from a manifest, add the app's secrets to the
//! .env file, and update the app when the manifest changes.

#![warn(clippy::missing_docs_in_private_items)]
#![warn(missing_docs)]

use anyhow::Error;
use async_trait::async_trait;
use clap::Parser;

use crate::cli::{Args, Command};
use crate::register::RegisterCommand;

mod cli;
mod manifest;
mod register;

/// Execute a command
///
/// This trait must be implemented by the subcommands of the command-line tool. It provides a single
/// method, `execute`, that will be called by the main function to run the command.
#[async_trait]
trait Execute {
    async fn execute(&self, global_args: &Args) -> Result<(), Error>;
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let global_args = Args::parse();

    let command: Box<dyn Execute> = match global_args.command() {
        Command::Register(args) => Box::new(RegisterCommand::new(args)),
    };

    command.execute(&global_args).await
}
