//! The form for starting the registration process

use askama_axum::Template;

use crate::manifest::SerializedManifest;

/// The form for starting the registration process
///
/// To register a new GitHub App, the user must send a POST request to the server with the manifest.
/// This is done by submitting a form in the browser so that the user is authenticated with GitHub.
/// This struct defines the fields of the form and renders the template.
#[derive(Clone, Eq, PartialEq, Debug, Template)]
#[template(path = "form.html", escape = "none")]
pub struct Form {
    /// The manifest for the GitHub App
    manifest: SerializedManifest,
}

impl Form {
    /// Create a new instance of the form
    pub fn new(manifest: SerializedManifest) -> Self {
        Self { manifest }
    }
}
