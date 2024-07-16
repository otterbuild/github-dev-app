//! A GitHub App's secrets and private key

use getset::{CopyGetters, Getters};
use serde::Deserialize;
use typed_fields::{name, number, secret};

number!(Id);
name!(ClientId);
secret!(ClientSecret);
secret!(WebhookSecret);
secret!(PrivateKey);

/// A GitHub App's secrets and private key
///
/// This struct represents a GitHub App's secrets and private key. The secrets are used to
/// authenticate the app with GitHub and the private key is used to sign requests.
#[derive(Clone, Debug, Deserialize, CopyGetters, Getters)]
#[allow(unused)] // TODO Remove when the secrets are saved locally
pub struct App {
    /// The unique identifier for the app
    #[getset(get_copy = "pub")]
    id: Id,

    /// The client ID for the app
    #[getset(get = "pub")]
    client_id: ClientId,

    /// The client secret for the app
    #[getset(get = "pub")]
    client_secret: ClientSecret,

    /// The webhook secret for the app
    #[getset(get = "pub")]
    webhook_secret: WebhookSecret,

    /// The private key for the app
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
