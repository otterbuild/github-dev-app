//! Command to register a new GitHub App

use anyhow::Error;
use async_trait::async_trait;

use crate::cli::Args;
use crate::manifest::Manifest;
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
}

#[async_trait]
impl<'a> Execute for RegisterCommand<'a> {
    async fn execute(&self, _global_args: &Args) -> Result<(), Error> {
        let manifest = Manifest::from_file(self.args.manifest())?;

        println!("{}", serde_json::to_string(&manifest)?);

        Ok(())
    }
}
