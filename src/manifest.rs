//! Manifest for a GitHub App

use std::collections::HashMap;
use std::path::Path;

use anyhow::{Context, Error};
use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use typed_fields::name;

name!(Name);
name!(Description);
name!(HomepageUrl);
name!(WebhookUrl);
name!(RedirectUrl);
name!(CallbackUrl);
name!(SetupUrl);
name!(Event);

name!(SerializedManifest);

/// Manifest for a GitHub App
///
/// GitHub Apps can be created through GitHub's API by providing a manifest for the app. The
/// manifest contains the configuration for the app, including its name, permissions, and callback
/// URLs.
///
/// The manifest has to be provided by the user. To make customization easier, some fields in the
/// manifest can be overwritten using command-line arguments when registering the app.
#[derive(Clone, Eq, PartialEq, Debug, Getters, Setters, Deserialize, Serialize, TypedBuilder)]
pub struct Manifest {
    /// The name of the GitHub App
    #[builder(default, setter(strip_option))]
    #[getset(get = "pub", set = "pub")]
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<Name>,

    /// The homepage of your GitHub App
    #[getset(get = "pub")]
    url: HomepageUrl,

    /// The configuration of the GitHub App's webhook
    #[builder(default, setter(strip_option))]
    #[getset(get = "pub")]
    #[serde(skip_serializing_if = "Option::is_none")]
    hook_attributes: Option<HookAttributes>,

    /// The full URL to redirect to after a user initiates the registration of a GitHub App from a
    /// manifest
    #[builder(default, setter(strip_option))]
    #[getset(get = "pub", set = "pub")]
    #[serde(skip_serializing_if = "Option::is_none")]
    redirect_url: Option<RedirectUrl>,

    /// A full URL to redirect to after someone authorizes an installation
    ///
    /// Up to 10 callback URLs can be provided.
    #[builder(default, setter(strip_option))]
    #[getset(get = "pub")]
    #[serde(skip_serializing_if = "Option::is_none")]
    callback_urls: Option<Vec<CallbackUrl>>,

    /// A full URL to redirect users to after they install your GitHub App if additional setup is
    /// required
    #[builder(default, setter(strip_option))]
    #[getset(get = "pub")]
    #[serde(skip_serializing_if = "Option::is_none")]
    setup_url: Option<SetupUrl>,

    /// A description of the GitHub App
    #[builder(default, setter(strip_option))]
    #[getset(get = "pub", set = "pub")]
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<Description>,

    /// Set to `true` when your GitHub App is available to the public or `false` when it is only
    /// accessible to the owner of the app
    #[builder(default, setter(strip_option))]
    #[getset(get = "pub")]
    #[serde(skip_serializing_if = "Option::is_none")]
    public: Option<bool>,

    /// The list of events the GitHub App subscribes to
    #[builder(default, setter(strip_option))]
    #[getset(get = "pub")]
    #[serde(skip_serializing_if = "Option::is_none")]
    default_events: Option<Vec<Event>>,

    /// The set of permissions needed by the GitHub App
    ///
    /// The format of the object uses the permission name for the key (for example, issues) and the
    /// access type for the value (for example, write).
    #[builder(default, setter(strip_option))]
    #[getset(get = "pub")]
    #[serde(skip_serializing_if = "Option::is_none")]
    default_permissions: Option<HashMap<String, String>>,

    /// Set to `true` to request the user to authorize the GitHub App, after the GitHub App is
    /// installed
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[getset(get = "pub")]
    request_oauth_on_install: Option<bool>,

    /// Set to `true` to redirect users to the setup_url after they update the GitHub installation
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[getset(get = "pub")]
    setup_on_update: Option<bool>,
}

/// Configuration of the GitHub App's webhook
///
/// The webhook configuration specifies the URL of the server that will receive the webhook `POST`
/// requests and whether the webhook is active.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Getters, Deserialize, Serialize)]
pub struct HookAttributes {
    /// The URL of the server that will receive the webhook `POST` requests
    #[getset(get = "pub")]
    url: WebhookUrl,

    /// Deliver event details when this hook is triggered, defaults to true
    #[getset(get = "pub")]
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
}

impl Manifest {
    /// Initialize a manifest from a file
    ///
    /// This method initializes a manifest from a file. The file is expected to contain a JSON
    /// object that will get deserialized into a manifest.
    pub fn from_file(path: &Path) -> Result<Self, Error> {
        let source = std::fs::read_to_string(path).context("failed to read manifest file")?;

        Self::from_str(&source)
    }

    /// Initialize a manifest from a string
    ///
    /// This method initializes a manifest from a string. The string is expected to be a JSON object
    /// that will get deserialized into a manifest.
    pub fn from_str(source: &str) -> Result<Self, Error> {
        serde_json::from_str(source).context("failed to deserialize manifest")
    }
}

impl TryFrom<Manifest> for SerializedManifest {
    type Error = Error;

    fn try_from(manifest: Manifest) -> Result<Self, Self::Error> {
        serde_json::to_string(&manifest)
            .context("failed to serialize manifest")
            .map(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use indoc::indoc;
    use pretty_assertions::assert_eq;
    use tempfile::NamedTempFile;

    use super::*;

    const JSON: &str = indoc! {r#"
            {
               "name": "Octoapp",
               "url": "https://www.example.com",
               "hook_attributes": {
                 "url": "https://example.com/github/events"
               },
               "redirect_url": "https://example.com/redirect",
               "callback_urls": [
                 "https://example.com/callback"
               ],
               "public": true,
               "default_permissions": {
                 "issues": "write",
                 "checks": "write"
               },
               "default_events": [
                 "issues",
                 "issue_comment",
                 "check_suite",
                 "check_run"
               ]
            }
        "#};

    #[test]
    fn from_file() {
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(JSON.as_bytes()).unwrap();

        let manifest = Manifest::from_file(file.path()).unwrap();

        assert_eq!(&Some("Octoapp".into()), manifest.name());
    }

    #[test]
    fn from_file_errors_on_empty_file() {
        let file = NamedTempFile::new().unwrap();

        let error = Manifest::from_file(file.path()).unwrap_err();

        assert!(format!("{:?}", error).contains("EOF while parsing a value"));
    }

    #[test]
    fn from_file_errors_on_missing_file() {
        let error = Manifest::from_file(Path::new("missing.json")).unwrap_err();

        assert!(format!("{:?}", error).contains("No such file or directory"));
    }

    #[test]
    fn from_str() {
        let manifest = Manifest::from_str(JSON).unwrap();

        assert_eq!(&Some("Octoapp".into()), manifest.name());
    }

    #[test]
    fn from_str_errors_on_invalid_json() {
        let json = r#"{"name": "Octoapp"}"#;

        let error = Manifest::from_str(json).unwrap_err();

        assert!(format!("{:?}", error).contains("missing field `url`"));
    }

    #[test]
    fn trait_deserialize() {
        let manifest: Manifest = serde_json::from_str(JSON).unwrap();

        assert_eq!(&Some("Octoapp".into()), manifest.name());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Manifest>();
    }

    #[test]
    fn trait_serialize() {
        let manifest = Manifest::builder()
            .url("https://www.example.com".into())
            .build();

        let expected = r#"{"url":"https://www.example.com"}"#;

        let json = serde_json::to_string(&manifest).unwrap();

        assert_eq!(expected, json);
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Manifest>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<Manifest>();
    }
}
