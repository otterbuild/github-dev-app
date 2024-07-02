//! Web server to accept the temporary code from GitHub

use std::net::SocketAddr;

use anyhow::{Context, Error};
use axum::extract::{Query, State};
use axum::routing::post;
use axum::Router;
use serde::Deserialize;
use tokio::net::TcpListener;
use tokio::sync::mpsc::{channel, Receiver, Sender};

/// Query parameters for the temporary code callback
///
/// This struct is used to deserialize the query parameters from the temporary code callback. The
/// callback is called after the user has registered a new GitHub App from a manifest, and can be
/// used to get the app's private key and secrets from GitHub.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Deserialize)]
pub(super) struct Params {
    /// The temporary code returned by GitHub
    code: String,
}

/// Start a web server on a background thread
///
/// This function starts a web server that accepts the temporary code from GitHub when a new GitHub
/// App is registered. The server listens on a random port and sends the temporary code to a channel
/// for further processing.
///
/// The server runs in a background task as to not block the main thread.
pub async fn spawn_web_server_on_background_thread() -> (SocketAddr, Receiver<String>) {
    let listener = TcpListener::bind("0.0.0.0:0".parse::<SocketAddr>().unwrap())
        .await
        .expect("failed to bind random port");
    let addr = listener.local_addr().unwrap();

    let (sender, receiver) = channel(1);

    let _server = tokio::spawn(run_axum_server(sender, listener));

    (addr, receiver)
}

/// Start the axum server to accept the temporary code
///
/// This function starts an axum server that listens for a POST request on the root path. The server
/// will accept the temporary code from the query parameters and send it to a channel for further
/// processing.
///
/// Spawn this function in a tokio task to start the server in the background.
async fn run_axum_server(channel: Sender<String>, listener: TcpListener) -> Result<(), Error> {
    let app = Router::new()
        .route("/", post(accept_temporary_code))
        .with_state(channel);

    axum::serve(listener, app)
        .await
        .context("failed to start axum server")?;

    Ok(())
}

/// Handle the callback after registering a new GitHub App
///
/// After registering a new GitHub App from a manifest, GitHub will redirect the user back to a
/// callback URL. The redirect includes a temporary code that can be exchanged for the app's private
/// key and secrets. This function accepts the temporary code and sends it to a channel for further
/// processing.
pub(super) async fn accept_temporary_code(channel: State<Sender<String>>, query: Query<Params>) {
    channel.send(query.code.clone()).await.unwrap()
}

#[cfg(test)]
mod tests {
    use reqwest::Client;

    use super::*;

    #[tokio::test]
    async fn sends_code_to_channel() {
        let (addr, mut receiver) = spawn_web_server_on_background_thread().await;

        let _response = Client::new()
            .post(format!("http://{}?code=otters-are-the-cutest", addr))
            .send()
            .await
            .expect("failed to execute POST / request");

        assert_eq!(Some("otters-are-the-cutest".into()), receiver.recv().await);
    }
}
