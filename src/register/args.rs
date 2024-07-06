//! Command-line arguments for the `register` subcommand

use std::path::PathBuf;

use clap::Parser;
use getset::{CopyGetters, Getters};
use typed_builder::TypedBuilder;

/// Command-line arguments for the `register` subcommand
///
/// The `register` subcommand is used to register a new GitHub App from a manifest file. The command
/// requires the path to the manifest file as an argument, and optionally accepts other arguments to
/// customize the manifest.
#[derive(
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Debug,
    Default,
    Parser,
    CopyGetters,
    Getters,
    TypedBuilder,
)]
pub struct RegisterArgs {
    /// The path to the manifest file
    #[arg()]
    #[builder(setter(into))]
    #[getset(get = "pub")]
    manifest: PathBuf,

    /// The port used by the embedded web server
    #[arg(long)]
    #[builder(setter(into))]
    #[getset(get_copy = "pub")]
    port: Option<u16>,
}
