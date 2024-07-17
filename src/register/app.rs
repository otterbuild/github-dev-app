//! A GitHub App's secrets and private key

use getset::{CopyGetters, Getters};
use serde::Deserialize;
#[cfg(test)]
use typed_builder::TypedBuilder;
use typed_fields::{name, number, secret};

number!(Id);
name!(Name);
name!(ClientId);
secret!(ClientSecret);
secret!(WebhookSecret);
secret!(PrivateKey);

/// A GitHub App's secrets and private key
///
/// This struct represents a GitHub App's secrets and private key. The secrets are used to
/// authenticate the app with GitHub and the private key is used to sign requests.
#[derive(Clone, Debug, Deserialize, CopyGetters, Getters)]
#[cfg_attr(test, derive(TypedBuilder))]
#[allow(unused)] // TODO Remove when the secrets are saved locally
pub struct App {
    /// The unique identifier for the app
    #[cfg_attr(test, builder(setter(into)))]
    #[getset(get_copy = "pub")]
    id: Id,

    /// The unique name for the app
    #[cfg_attr(test, builder(setter(into)))]
    #[getset(get = "pub")]
    name: Name,

    /// The client ID for the app
    #[cfg_attr(test, builder(setter(into)))]
    #[getset(get = "pub")]
    client_id: ClientId,

    /// The client secret for the app
    #[cfg_attr(test, builder(setter(into)))]
    #[getset(get = "pub")]
    client_secret: ClientSecret,

    /// The webhook secret for the app
    #[cfg_attr(test, builder(setter(into, strip_option)))]
    #[getset(get = "pub")]
    webhook_secret: Option<WebhookSecret>,

    /// The private key for the app
    #[cfg_attr(test, builder(setter(into)))]
    #[getset(get = "pub")]
    pem: PrivateKey,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<App>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<App>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<App>();
    }
}
